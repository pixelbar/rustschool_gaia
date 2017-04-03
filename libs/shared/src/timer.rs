use time::precise_time_ns;

pub struct Timer {
    time: u64,
}

impl Timer {
    pub fn start() -> Timer {
        Timer {
            time: precise_time_ns()
        }
    }

    pub fn print_elapsed(&mut self, format: &'static str) {
        let time = precise_time_ns();
        let diff = time - self.time;
        self.time = time;
        println!("{}: {}ms", format, (diff as f32 / 1_000_000f32));
    }
}