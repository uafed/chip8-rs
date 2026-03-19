# Chip8 Interpreter

This is a re-implementation of a previous project that I worked on in order to learn Rust.

The GIF below shows the "Sierpinski Triangle" demo by Sergey Naydenov (2010) [chip-8-roms](https://github.com/kripod/chip8-roms). This is in progress since a lot of the instructions are not yet supported. The GIF also shows the performance with the FPS capped to a lower value just for inspection.

![Sierpinski demo](demo/output.gif)

## Calling Convention

`V1` - return value
`V2` - 1st argument
`V3` - 2nd argument
...
