extern crate shared;

use shared::{Timer, GaiaEntry, generate_image};

fn main() {
    let mut timer = Timer::start();
    let source = include_str!("../TgasSource_000-000-000.csv");
    timer.print_elapsed("Loaded csv in memory");

    let mut is_first_line = true;
    let mut line_nr = 1;
    let mut entries = Vec::new();

    for line in source.lines() {
        if is_first_line {
            GaiaEntry::validate_headers(line);
            is_first_line = false;
            continue;
        }

        let entry = GaiaEntry::from_line(line);
        if entry.is_none() {
            println!("Could not read line {:?}:", line_nr);
            println!("{:?}", line);
            panic!();
        }
        let entry = entry.unwrap();
        entries.push(entry);

        line_nr += 1;

        // if line_nr > 1000 {
        //     break;
        // }
    }
    timer.print_elapsed("Parsed csv");

    generate_image("out.png", entries);
    timer.print_elapsed("Generated image");
}
