use gaia_entry::GaiaEntry;
use image::{ImageBuffer, Rgb};

pub fn generate_image(filename: &str, entries: Vec<GaiaEntry>) {
    let mut min_ra = ::std::f64::MAX;
    let mut max_ra = ::std::f64::MIN;
    let mut min_dec = ::std::f64::MAX;
    let mut max_dec = ::std::f64::MIN;

    for entry in &entries {
        min_ra = min_ra.min(entry.ra);
        max_ra = max_ra.max(entry.ra);
        min_dec = min_dec.min(entry.dec);
        max_dec = max_dec.max(entry.dec);
    }

    let mut map = box [[0u16;600];800];

    let ra_factor = 799f64 / (max_ra - min_ra);
    let dec_factor = 599f64 / (max_dec - min_dec);
    let mut highest: u16 = 0;

    for entry in &entries {
        let x = ((entry.ra - min_ra) * ra_factor) as usize;
        let y = ((entry.dec - min_dec) * dec_factor) as usize;
        let count = map[x][y] + 1;
        map[x][y] = count;
        highest = if highest > count { highest } else { count };
    }

    let factor = (highest as f64) / 255f64;

    let image = ImageBuffer::from_fn(800, 600, |x, y| {
        let count = map[x as usize][y as usize];
        let value = ((count as f64) / factor) as u8;
        Rgb { data: [value, value, value] }
    });

    image.save(filename).unwrap();
} 