# bracket-terminal

`bracket-terminal` is part of the `bracket-lib` family. It provides a virtual ASCII/Codepage-437 terminal (with optional tile graphic support and layers), and a game loop. This frees you up from implementation difficulties, making it easy to write grid-based games (Roguelikes are a great fit, but any grid/tile-based game can work). It also provides assistance with keyboard and mouse input.

Bracket-terminal supports multiple back-ends:

* The default is `OpenGL`, which works on just about everything. The GL back-end supports all features, including post-processing (retro screen effects) and layers.
* The `WebGL` (WASM) back-end works in Web Assembly, allowing you to compile your `bracket-terminal`-based game for the web.
* The `Amethyst` back-ends provide rendering in `Vulkan` and `Metal`. It currently supports everything except the post-processing effects.
* The `crossterm` back-end runs natively in your existing terminal. Graphical features are not supported.
* The `curses` back-end runs natively in *NIX terminals, or in a `pdcurses` terminal emulator on Windows. Graphical features are not supported.

## Why `bracket-terminal` and not direct console rendering?

Bracket-terminal can do terminal rendering, but if that is your only target you may be better off using `crossterm`. Bracket-terminal gets you a few features you don't find elsewhere:

* It is game-loop based, so it is ideal for frame-oriented game programming.
* Codepage-437 emulation is sprite-based on graphical back-ends. You can be absolutely sure that your game will look the same on all platforms, using *exactly* the font(s) you specify.
* It provides multiple layers, which can use different font/sprite files.
* There are some retro post-processing effects available if you like them.
* `bracket-terminal` works hard to be simple and straightforward, making for a great learning environment.

## Minimal example

The following code is enough to put `Hello Minimal Bracket World` on the screen:

```rust
use bracket_terminal::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Hello Bracket World");
    }
}

fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build();

    let gs: State = State {};
    main_loop(context, gs);
}
```

It's worth noting that `(0,0)` in `bracket-terminal` is the top-left of the screen.

## Examples

* `cargo run --example hello_minimal` puts "Hello Minimal Bracket World" on the screen.
* `cargo run --example hello_terminal` puts a bouncing "Hello World" on the screen in color, with frames-per-second [FPS] counting, and frame-rate limiting.
* `cargo run --example sparse` is the same demo, but with a second layer in a VGA 8x16 font on a second layer, no frame-rate limiting, and utilizing batched command submission.
* `cargo run --example walking` lets you use your keyboard to walk an `@` symbol around a random map.
* `cargo run --example astar-mouse` lets you use your mouse to move around a random map, using A-Star pathing (from the `bracket-pathfinding` crate) to avoid obstacles.
* `cargo run --example tiles` is similar to the `walking` demo, but uses two layers of graphical tiles (graphical back-ends only).
* `cargo run --example rex` demonstrates loading a sprite from [REX Paint](https://www.gridsagegames.com/rexpaint/) and rendering it to the terminal.
* `cargo run --example postprocess` demonstrates the library's post-processing effects - scan lines and screen burn.
* `cargo run --example textblock` demonstrates the `TextBlock` system, giving you a "builder" approach to constructing larger blocks of text with word-wrapping and formatting.
* `cargo run --example dwarfmap` demonstrates using the terminal with `Algorithm3D` to provide a Dwarf Fortress style 3D map (2D "slices" of a 3D world). It uses the `bracket-noise` library for terrain generation.
* `cargo run --example keyboard` demonstrates keyboard scan-code input. It's mostly useful for debugging.
* `cargo run --example textsprites` demonstrates multi-tile sprites.
