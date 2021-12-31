//! Proxy the microphone to vb cable
//! Strongly copied from cpal example "feedback"

use ringbuf::RingBuffer;
use rodio::cpal;
use rodio::cpal::traits::{HostTrait, StreamTrait};
use rodio::DeviceTrait;
use std::time::Duration;

pub fn start() {
    std::thread::spawn(_start);
}

/// Take the stream from the microphone (default_input) and proxy it to the vb cable output.
pub fn _start() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let output_device = crate::get_vb_cable();

    if output_device.is_none() {
        return Ok(()); // No proxying done without VB Cable
    }

    let vb_cable = output_device.unwrap();
    // Find devices.
    let mic = host
        .default_input_device()
        .expect("failed to find input device");

    // We'll try and use the same configuration between streams to keep it simple.
    let config: cpal::StreamConfig = mic.default_input_config()?.into();

    // Create a delay in case the input and output devices aren't synced.
    let latency_frames = (150.0 / 1000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    // The buffer to share samples
    let ring = RingBuffer::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.push(0.0).unwrap();
    }

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;
        for &sample in data {
            if producer.push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            log::error!("output stream fell behind: try increasing latency");
        }
    };

    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }
        if input_fell_behind {
            log::error!("input stream fell behind: try increasing latency");
        }
    };

    // Build streams.
    log::info!(
        "Attempting to build both streams with f32 samples and `{:?}`.",
        config
    );
    let input_stream = mic.build_input_stream(&config, input_data_fn, err_fn)?;
    let output_stream = vb_cable.build_output_stream(&config, output_data_fn, err_fn)?;
    log::info!("Successfully built streams.");

    input_stream.play()?;
    output_stream.play()?;

    loop {
        std::thread::sleep(Duration::from_secs(600)) // avoid CPU cycles
    }
}

fn err_fn(err: cpal::StreamError) {
    log::error!("an error occurred on stream: {}", err);
}
