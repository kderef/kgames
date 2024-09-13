# KGames  

*A Spritual successor to [Multisim](https://github.com/kderef/multisim).*  

The goal is to create a scripting engine for making your own games.
This is to be achieved by exposing a few [macroquad](https://macroquad.rs) functions through [Rhai script](https://thai.red).
and allowing the user to look through the detected scripts in a graphical menu.

### Getting Started  

Install or build the executable, then run it once to create the neccesessary directories.  
You should now see that a folder called **kgames** has been created, with the following inside:
- README.txt: this readme file
- scripts/:   The folder containing all the user-made scripts
- examples/:  The folder containing builtin examples, these can be modified(!)


#### THE RHAI LANGUAGE
**Links:**
- https://www.rhai.rs
- https://www.rhai.rs/book/

The rhai programming language is a scripting language based on- and embeddable in RUST
Rust is the language that kgames is implemented in, find out more at https://www.rust-lang.org
Guides and information for the rhai are available at https://www.rhai.rs
Since rhai is based on rust, the syntax is quite similar:

**syntax:**
- Declare variables with 'let'
- Declare functions with 'fn' (these always have an implicit *this* parameter)
- Blocks (if, else, fn) are started and ended with { and }
- Constants are declared with 'const'
- Statements are terminated with a semicolon (';')

