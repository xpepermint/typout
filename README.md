> Typewriter output stream.

This is a simple output writer supporting memory and console streamlets out of the box.

## Example

```rs
let mut out = Typout::console();
out.write("a");
out.debug("b");
out.info("c");
out.warn("d");
out.error("e");
out.flush(); // -> abcde
```
