# bracket-terminal

`bracket-terminal` is part of the `bracket-lib` family. It provides a virtual ASCII/Codepage-437 terminal (with optional tile graphic support and layers), and a game loop. This frees you up from implementation difficulties, making it easy to write grid-based games (Roguelikes are a great fit, but any grid/tile-based game can work). It also provides assistance with keyboard and mouse input.

Bracket-terminal supports multiple back-ends:

* The default is `OpenGL`, which works on just about everything. The GL back-end supports all features, including post-processing (retro screen effects) and layers.
* The `WebGL` (WASM) back-end works in Web Assembly, allowing you to compile your `bracket-terminal`-based game for the web.
* The `Amethyst` back-ends provide rendering in Vulkan and Metal. It currently supports everything except the post-processing effects.
* The `crossterm` back-end runs natively in your existing terminal. Graphical features are not supported.
* The `curses` back-end runs natively in *NIX terminals, or in a "pdcurses" terminal emulator on Windows. Graphical features are not supported.

## Why `bracket-terminal` and not direct console rendering?

Bracket-terminal can do terminal rendering, but if that is your only target you may be better off using `crossterm`. Bracket-terminal gets you a few features you don't find elsewhere:

* It is game-loop based, so it is ideal for frame-oriented game programming.
* Codepage-437 emulation is sprite-based on graphical back-ends. You can be absolutely sure that your game will look the same on all platforms, using *exactly* the font you specify.
* `bracket-terminal` works hard to be simple and straightforward, making for a great learning environment.

