#pragma once

#include "rl.hh"

class Game {
public:
    bool exit = false;
    const char* title;

    Game() {}
    virtual ~Game() {}
    virtual void update() = 0;
    virtual void draw() = 0;
    virtual void reset() = 0;
    virtual const Texture& icon() = 0;
};