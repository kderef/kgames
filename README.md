# KGames  

Spritual successor to [Multisim](https://github.com/kderef/multisim).  

The goal is to create a scripting engine for making your own games
this is to be achieved by exposing a few [macroquad](https://macroquad.rs) functions through [Rhai script](https://rhai.rs)
and allowing the user to look through the detected scritps in a graphical menu.
---

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
