extern crate typout;

use std::time::Instant;
use std::time::Duration;
use typout::{Typout, Spinner};

fn main() {
    let now = Instant::now();

    let mut spinner = Spinner::default();
    spinner.set_frames(vec!["|", "/", "-", "\\"]);
    spinner.set_speed(50);
    let mut out = Typout::default();
    out.set_spinner(spinner);

    loop {
        out.write("Tick");
        out.write(&format!(" started {:?} seconds ago\n", now.elapsed().as_secs()));
        out.flush();

        out.pin("a", "Pin message A\n");
        std::thread::sleep(Duration::from_secs(1));
        out.pin("b", "Pin message B\n");
        std::thread::sleep(Duration::from_secs(1));
        out.pin("a", "Pin message A (updated)\n");
        std::thread::sleep(Duration::from_secs(1));
        out.pin("b", "Pin message B (updated)\n");
        std::thread::sleep(Duration::from_secs(1));
        out.unpin("a");
        std::thread::sleep(Duration::from_secs(1));
        out.unpin("b");

        std::thread::sleep(Duration::from_secs(1));
        out.spin("a", "Loading .\n");
        std::thread::sleep(Duration::from_secs(1));
        out.spin("b", format!("Progress {:.1}% ..\n", 50));
        std::thread::sleep(Duration::from_secs(1));
        out.spin("a", "Loading ..\n");
        std::thread::sleep(Duration::from_secs(1));
        out.spin("b", format!("Progress {:.1}% ..\n", 100));
        std::thread::sleep(Duration::from_secs(1));
        out.unpin("a");
        std::thread::sleep(Duration::from_secs(1));
        out.unpin("b");
        std::thread::sleep(Duration::from_secs(1));
    }
}
