extern crate shared;

use shared::{Timer, GaiaEntry, generate_image};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut timer = Timer::start();
    let handle = File::open("TgasSource_000-000-000.csv");
    
    let mut handle = match handle {
        Ok(handle) => handle,
        Err(err) => {
            println!("Error opening file: {}", err);
            return;
        }
    };

    let bufreader = BufReader::new(handle);

    let mut is_first_line = true;
    let mut line_nr = 1;
    let mut entries = Vec::new();

    for line in bufreader.lines() {
        let line = line.unwrap();
        if is_first_line {
            GaiaEntry::validate_headers(&line);
            is_first_line = false;
            continue;
        }

        let entry = GaiaEntry::from_line(line.clone());
        if entry.is_none() {
            println!("Could not read line {:?}:", line_nr);
            println!("{:?}", line);
            panic!();
        }
        let entry = entry.unwrap();
        entries.push(entry);

        line_nr += 1;

        if line_nr > 1000 {
            break;
        }
    }
    timer.print_elapsed("Parsed csv");

    generate_image("out.png", entries);
    timer.print_elapsed("Generated image");
}
