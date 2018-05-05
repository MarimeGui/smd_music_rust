extern crate clap;
extern crate smd_music;

use clap::{App, Arg};
use smd_music::smd::track::event::Event;
use smd_music::smd::SMD;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let matches = App::new("SMD/SMD File Info")
        .version("0.1")
        .author("Marime Gui")
        .about("Gives basic info about an SMD/SMD File")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the file to use")
                .required(true)
                .index(1),
        )
        .get_matches();
    let reader = &mut BufReader::new(File::open(matches.value_of("INPUT").unwrap()).unwrap());
    let smd = SMD::import(reader).unwrap();
    println!("File Name: {}", smd.header.name);
    println!(
        "{} tracks, {} channels",
        smd.song.nb_tracks, smd.song.nb_channels
    );
    for track in smd.tracks {
        println!(
            "Track ID {}, Channel ID {}, {} events",
            track.track_id,
            track.channel_id,
            track.events.len()
        );
        let mut nb_noteplay = 0u32;
        let mut nb_delta_time = 0u32;
        let mut nb_wait = 0u32;
        for event in track.events {
            match event {
                Event::NotePlay(_) => nb_noteplay += 1,
                Event::DeltaTime(_) => nb_delta_time += 1,
                Event::Wait1Byte(_)
                | Event::Wait2Byte(_)
                | Event::WaitAdd(_)
                | Event::WaitAgain => nb_wait += 1,
                _ => {}
            }
        }
        println!("    {} Note Play events", nb_noteplay);
        println!("    {} Delta Time events", nb_delta_time);
        println!("    {} Wait events", nb_wait);
    }
}
