# hello_midi

Starter project for MIDI with rust.

## Usage

We recommend downloading the JS Bach Goldberg Variations from https://www.opengoldbergvariations.org/.

```sh
cargo run ~/Documents/Goldberg_Variations.mid
```

## Mac Config

* Install SimpleSynth according to https://github.com/wbsoft/frescobaldi/wiki/MIDI-playback-on-Mac-OS-X

## MIDI sample data

We ran [rimd](https://github.com/RustAudio/rimd) test bin against the Goldberg Variations and found a lot of Note On events.

```sh
% cargo run --bin test # rimd

Reading: /Documents/Goldberg_Variations.mid
format: multiple track
tracks: 2
division: 480

1: Track, copyright: [none], name: [none]
events:
  time: 0	Meta Event: Time Signature: 3/4, 24 ticks/metronome click, 8 32nd notes/quarter note
  time: 0	Meta Event: Key Signature, 1 sharps/flats, Major
  time: 0	Meta Event: Set Tempo, microseconds/quarter note: 833333
  time: 0	Control Change: [121,0]	channel: Some(0)
  time: 0	Program Change: [1]	channel: Some(0)
  time: 0	Control Change: [7,127]	channel: Some(0)
  time: 0	Control Change: [10,64]	channel: Some(0)
  time: 0	Control Change: [91,0]	channel: Some(0)
  time: 0	Control Change: [93,0]	channel: Some(0)
  time: 0	Meta Event: MIDI Port Prefix Assignment, port: 0
  time: 0	Note On: [79,80]	channel: Some(0)
  time: 479	Note On: [79,0]	channel: Some(0)
  time: 480	Note On: [79,80]	channel: Some(0)
  time: 959	Note On: [79,0]	channel: Some(0)
```

...

```sh
  time: 3715	Note On: [67,0]	channel: Some(0)
  time: 3720	Note On: [69,80]	channel: Some(0)
  time: 3778	Note On: [67,80]	channel: Some(0)
  time: 3784	Note On: [69,0]	channel: Some(0)
  time: 3839	Note On: [67,0]	channel: Some(0)
  time: 3839	Note On: [69,80]	channel: Some(0)
  time: 3899	Note On: [67,80]	channel: Some(0)
  time: 3900	Note On: [69,0]	channel: Some(0)
  time: 3957	Note On: [67,0]	channel: Some(0)
  time: 3957	Note On: [69,80]	channel: Some(0)
```

We can see by reading http://www.onicos.com/staff/iz/formats/midi-event.html that most of these are "Channel 1 Note On".

## Acknowledgements

Bach's Goldberg Variations are [available under Creative Commons License here](https://www.opengoldbergvariations.org/).

Big thanks to [midir library](https://github.com/Boddlnagg/midir).
