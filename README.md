> Typewriter output stream.

## Example

```rs
let mut out = Console::default();
// getters
out.speed();
out.chars();
out.verbosity();
out.stripansi(false);
// setters
out.set_spinner_speed(30);
out.set_spinner_chars(vec!["|", "-"]);
out.set_verbosity(Verbosity::Debug);
out.set_stripansi(false);
// operations
out.write("a");
out.debug("b");
out.info("c");
out.warn("d");
out.error("e");
out.flush(); // -> abcde
out.spin("Loading ..."); // animation
out.progress(100, 20, "Loading ..."); // animation
```
