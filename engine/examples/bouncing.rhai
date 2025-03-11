let state = #{
    pos: vec2(100.0, 100.0),
    vel: vec2(200.0, 200.0),
    yes: get_texture("yes"),
    no: get_texture("no"),
};

const SIZE = vec2(128.0, 128.0);

fn draw_bouncy() {
    let tex;
    if this.vel.x < 0.0 {
        tex = this.yes;
    } else {
        tex = this.no;
    }
    texture(tex, this.pos.x, this.pos.y, WHITE);
}

fn update() {
    let width = screen_width();
    let height = screen_height();

    let dt = deltatime();
    state.pos.x += state.vel.x * dt;
    state.pos.y += state.vel.y * dt;

    if state.pos.x <= 0.0 || state.pos.x + SIZE.x >= width  {
        state.pos.x = state.pos.x.max(0.0).min(width);
        state.vel.x *= -1.0;
    }

    if state.pos.y <= 0.0 || state.pos.y + SIZE.y >= height {
        state.pos.y = state.pos.y.max(0.0).min(height);
        state.vel.y *= -1.0;
    }
}

fn draw() {
    clear(WIN98_BG);
    state.draw_bouncy();
}
