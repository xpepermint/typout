> Typewriter output stream.

This is a simple output writer supporting memory and console streamlets out of the box.

## Example

```rs
let mut out = Typout::console();
out.write("Hello");
out.write("World");
out.flush(); // -> Hello World
```
