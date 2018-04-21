extern crate clap;
extern crate smd_music;

use clap::{App, Arg};
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
    println!("Exported At {}", smd.header.export_date);
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
    }
}
