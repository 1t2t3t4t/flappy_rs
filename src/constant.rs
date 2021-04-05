pub const EMPTY_DRAW_PARAM: ([f32; 2],) = ([0f32, 0f32],);

pub mod color {
    use ggez::graphics::Color;

    pub const BLUE: Color = Color::new(0.529, 0.808, 0.922, 1.0);
}

pub mod world {
    pub const GRAVITY: f32 = 9.8;

    pub const JUMP_FORCE: f32 = -250f32;
    pub const BIRD_SIZE: f32 = 60f32;

    pub const PILLAR_WIDTH: f32 = 100f32;
    pub const PILLAR_SPEED: f32 = 150f32;
}
