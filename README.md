# KGames  

*A Spritual successor to [Multisim](https://github.com/kderef/multisim).*  

The goal is to create a scripting engine for making your own games.
This is to be achieved by exposing a few [macroquad](https://macroquad.rs) functions through [Rhai script](https://thai.red).
and allowing the user to look through the detected scripts in a graphical menu.


### Example  

Install or build the executable, then run it once to create the neccesessary directories.  
Then put the following in the file `kgames/scripts/moving_ball.rhai`:
```rust
let x = screen_width() / 2.0;
let y = screen_height() / 2.0;

fn update() {
    if key_down(KEY_RIGHT) {
        x += 1.0;
    }
    if key_down(KEY_LEFT) {
        x -= 1.0;
    }
    if key_down(KEY_DOWN) {
        y += 1.0;
    }
    if key_down(KEY_UP) {
        y -= 1.0;
    }
}

fn draw() {
    clear(BLACK);
    circle(x, y, 15.0, YELLOW);
    text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
}
```
Then press F5 or restart the app, and you should see an option for `moving_ball.rhai`.  
**TIP:** You can toggle terminal logs and FPS counter with F10.  
---
# Globals list  
All the **constants, functions and types** exposed to the scripts.

> TODO
---
KGAMES
======
> A simple script-based game engine, by github.com/kderef
> Using the rhai scripting language: https://rhai.rs

ARCHITECTURE
============
> Upon starting the game for the first time, it will attempt to create the
> following directory structure (if it doesn't exist already)
$ kgames/
$       scripts/
$       README.txt (this file)

> Then, it will attempt to load all the scripts from the kgames/scripts/ directory.
> IMPORTANT: Only files ending in .rhai will be loaded! any other will be skipped.

EXAMPLE
=======
> When starting for the first time, you will be greeted with a (mostly) empty screen,
> with just a title: "KGames"
> This means that the scripts/ folder has no .rhai scripts.

> To start create the file kgames/scripts/moving_ball.rhai
> then put the following in the file:

###### CODE ######
let x = screen_width() / 2.0;
let y = screen_height() / 2.0;

fn update() {
    if key_down(KEY_RIGHT) {
        x += 1.0;
    }
    if key_down(KEY_LEFT) {
        x -= 1.0;
    }
    if key_down(KEY_DOWN) {
        y += 1.0;
    }
    if key_down(KEY_UP) {
        y -= 1.0;
    }
}

fn draw() {
    clear(BLACK);
    circle(x, y, 15.0, YELLOW);
    text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
}
#### END CODE ####

> Then press F5 to reload the scripts, or simply restart the application.
> You should see an option for moving_ball.rhai
> Press on it, and you should see a YELLOW ball on a BLACK ball, you can move the ball with the ARROW KEYS.
> Then, with the game still open, make the following change to the line just below fn draw() { ...
$ clear(BLACK) --> clear(BLUE)
> Then save the file, and hotreload the script by pressing F5, and you should see the background turn BLUE!

THE RHAI LANGUAGE
=================
% LINKS %
https://www.rhai.rs
https://www.rhai.rs/book/

The rhai programming language is a scripting language based on- and embeddable in RUST
Rust is the language that kgames is implemented in, find out more at https://www.rust-lang.org
Guides and information for the rhai are available at https://www.rhai.rs
Since rhai is based on rust, the syntax is quite similar:

$ Declare variables with 'let'
$ Declare functions with 'fn'
$ Blocks (if, else, fn) are started and ended with { and }
$ Constants are declared with 'const'
$ Statements are terminated with a semicolon (';')

