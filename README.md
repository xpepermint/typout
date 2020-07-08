> Command-line typewriter output stream.

This crate provides a wrapper around the stdout and allows for writing messages to the terminal output.

## Usage

```rs
let mut out = Typout::default();

// write a simple message
out.write("Hello");
out.write(" World!");
out.flush(); // -> Hello World!

// pin/unpin a message to the end
out.pin("ID1", "Please wait ...");
out.unpin("ID1");

// spin/unpin an animated message to the end
out.spin("ID2", "Processing ...");
out.unpin("ID1");
```

Se it in action by running `cargo run --example simulate`.
