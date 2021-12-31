This is a binary that periodically queries data to the local LoL game client.

Using these data, it triggers some callbacks (for example, playing a sound to congratulate your friends on Discord if they made a kill).

The sounds must all be stored in the `sounds` folder, along with a `mapping.json` file. 
The format of this file is a simple json where each key correspond to a summoner name and the value in an array of strings, each representing the name of a file that should be played if this summoner makes a kill.

There are also the following additional keys:
- `ace`: Sounds to be played when our team scores an ace
- `aced`: Sounds to be played when the ennemy team scores an ace
- `game_start`: Sounds to be played at the beginning of the game.
- `default`: Sounds to be played if a kill is made by a player from our team, but it's name is not specified in the json.

For example, the following `mapping.json` is valid:
```json
{
  "ace": ["poggers.mp3"],
  "aced": ["sad_sound.mp3"],
  "game_start": ["lets_go.mp3"],
  "default": [],
  "KC Rekkles": ["omggg.mp3"]
}
```

## VB Cable
To be able to use this program output as an input for Discord, we will use a third party software: [VB Cable](https://vb-audio.com/Cable/).
This program will play its sound in the `VB Cable` output, and the `VB Cable` input will be used as a source for Discord. Thus this program also needs to proxy the voice from our microphone to the output of VB Cable.

The proxying is also doable via OBS and the "feedback" feature as can be seen [on this video](https://www.youtube.com/watch?v=Clcq7fk6L1k). If you want to use OBS, you can disable our voice proxying by disabling the default `voice_proxy` feature.  
This is what I ended up doing on Windows because I couldn't get my microphone and the vb cable to have compatible configs