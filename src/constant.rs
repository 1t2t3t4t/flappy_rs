pub const EMPTY_DRAW_PARAM: ([f32; 2],) = ([0f32, 0f32],);

pub mod color {
    use ggez::graphics::Color;

    pub const BLUE: Color = Color::new(0.529, 0.808, 0.922, 1.0);
}

pub mod world {
    pub const GRAVITY: f32 = 9.8;
}