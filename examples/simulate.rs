extern crate typout;

use std::time::Instant;
use std::time::Duration;
use typout::Typout;

fn main() {
    let mut out = Typout::default();
    let now = Instant::now();
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
        out.spin("b", "Progress .\n");
        std::thread::sleep(Duration::from_secs(1));
        out.spin("a", "Loading ..\n");
        std::thread::sleep(Duration::from_secs(1));
        out.spin("b", "Progress ..\n");
        std::thread::sleep(Duration::from_secs(1));
        out.unpin("a");
        std::thread::sleep(Duration::from_secs(1));
        out.unpin("b");
        std::thread::sleep(Duration::from_secs(1));
    }
}
