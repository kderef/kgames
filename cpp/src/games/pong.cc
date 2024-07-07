#pragma once

#include "../rl.hh"
#include "../game.cc"



class Pong : public Game {
private:
    rl::RenderTexture img;

public:
    Pong() {
        title = "Pong";
        img = rl::RenderTexture()
    }
    virtual const Texture& icon() {
        return img.texture;
    }
    virtual void draw() {
        ClearBackground(WHITE);
    }
};