extern crate midir;
extern crate rimd;

use midir::MidiOutput;
use rimd::{Event, SMFError, TrackEvent, SMF};
use std::env::{args, Args};
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

mod channels;

const DEFAULT_VEC_CAPACITY: usize = 133000;

#[derive(Debug)]
pub enum ChannelNote {
    On(u8),
    Off(u8),
}

impl ChannelNote {
    pub fn new(channel: u8) -> Option<ChannelNote> {
        if channel >= channels::CHANNEL_OFF_FIRST && channel <= channels::CHANNEL_OFF_LAST {
            Some(ChannelNote::Off(channel))
        } else if channel >= channels::CHANNEL_ON_FIRST && channel <= channels::CHANNEL_ON_LAST {
            Some(ChannelNote::On(channel))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct MidiNote {
    channel_note: ChannelNote,
    time: u64,
    vtime: u64,
    note: u8,
    velocity: u8,
}

fn main() {
    let mut args: Args = args();
    args.next();
    let pathstr = &match args.next() {
        Some(s) => s,
        None => panic!("Please pass a path to an SMF to test"),
    }[..];

    let (track_events, microsec_per_quarter_note) = load_midi_file(pathstr);

    println!("microsec per quarter note: {}", microsec_per_quarter_note);

    let timed_midi_messages = midi_messages_from(track_events);

    let notes = notes_in_channel(timed_midi_messages);

    match run(notes) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description()),
    }
}

fn load_midi_file(pathstr: &str) -> (Vec<TrackEvent>, u64) {
    let mut events: Vec<TrackEvent> = Vec::with_capacity(DEFAULT_VEC_CAPACITY);

    let mut micros_per_qnote: Option<u64> = None;

    match SMF::from_file(&Path::new(&pathstr[..])) {
        Ok(smf) => {
            for track in smf.tracks.iter() {
                for event in track.events.iter() {
                    if let rimd::Event::Meta(rimd::MetaEvent {
                        command: rimd::MetaCommand::TempoSetting,
                        length: _,
                        data: _,
                    }) = event.event
                    {
                        // so meta

                    }

                    println!("  {:?}", event);
                    events.push(event.clone());
                }
            }
        }
        Err(e) => match e {
            SMFError::InvalidSMFFile(s) => {
                println!("{}", s);
            }
            SMFError::Error(e) => {
                println!("io: {}", e);
            }
            SMFError::MidiError(e) => {
                println!("Midi Error: {}", e);
            }
            SMFError::MetaError(_) => {
                println!("Meta Error");
            }
        },
    };

    const DEFAULT_MICROS_PER_QNOTE: u64 = 888888888;
    (events, micros_per_qnote.unwrap_or(DEFAULT_MICROS_PER_QNOTE))
}

#[derive(Debug)]
pub struct TimedMidiMessage {
    pub vtime: u64,
    pub data: Vec<u8>,
}

fn midi_messages_from(track_events: Vec<TrackEvent>) -> Vec<TimedMidiMessage> {
    let mut midi_messages: Vec<TimedMidiMessage> = Vec::with_capacity(DEFAULT_VEC_CAPACITY);

    for te in track_events {
        match te {
            TrackEvent { vtime, event } => match event {
                Event::Midi(m) => midi_messages.push(TimedMidiMessage {
                    vtime,
                    data: m.data,
                }),
                Event::Meta(_m) => {}
            },
        }
    }

    midi_messages
}

fn notes_in_channel(midi_messages: Vec<TimedMidiMessage>) -> Vec<MidiNote> {
    let mut time: u64 = 0;
    let mut notes: Vec<MidiNote> = Vec::with_capacity(DEFAULT_VEC_CAPACITY);
    for msg in midi_messages {
        time += msg.vtime;
        if let Some(cn) = ChannelNote::new(msg.data[0]) {
            notes.push(MidiNote {
                channel_note: cn,
                time: time,
                vtime: msg.vtime,
                note: msg.data[1],
                velocity: msg.data[2],
            })
        }
    }

    notes
}

fn run(notes: Vec<MidiNote>) -> Result<(), Box<Error>> {
    let midi_out = MidiOutput::new("Hello MIDI Bach Magic Machine")?;

    // Get an output port (read from console if multiple are available)
    let out_port = match midi_out.port_count() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(0).unwrap()
            );
            0
        }
        _ => {
            println!("\nAvailable output ports:");
            for i in 0..midi_out.port_count() {
                println!("{}: {}", i, midi_out.port_name(i).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            input.trim().parse()?
        }
    };

    println!("\nOpening connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    println!("Connection open. Listen!");
    {
        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |midi: MidiNote| {
            // We're ignoring errors in here
            sleep(Duration::from_millis(midi.vtime * 2));

            let _ = match midi.channel_note {
                ChannelNote::On(c) => conn_out.send(&[c, midi.note, midi.velocity]),
                ChannelNote::Off(c) => conn_out.send(&[c, midi.note, midi.velocity]),
            };
        };

        for n in notes {
            play_note(n)
        }
    }

    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");
    Ok(())
}
