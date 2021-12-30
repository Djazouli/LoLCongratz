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

## TODO
Currently, the sound is played in your own headphones, and not on some virtual microphone that can be plugged into Discord.  
I think this will require installing 3rd party software to allow this.
