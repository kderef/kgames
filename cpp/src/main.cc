#include "rl.hh"
#include "games/pong.cc"

int main(void) {
    rl::Window window(800, 600, "Hello, World!");

    while (!window.ShouldClose()) {
        window.BeginDrawing();

    #if DEBUG
        window.DrawFPS();
    #endif
        window.EndDrawing();
    }

    return 0;
}