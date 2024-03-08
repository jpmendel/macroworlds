#[derive(Debug, Clone)]
pub struct Turtle {
    pub name: String,
    pub pos: (f32, f32),
    pub heading: f32,
    pub color: f32,
    pub is_visible: bool,
    pub is_drawing: bool,
}

impl Turtle {
    pub fn with(name: String) -> Self {
        Turtle {
            name,
            pos: (0.0, 0.0),
            heading: 0.0,
            color: 0.0,
            is_visible: true,
            is_drawing: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start: (f32, f32),
    pub end: (f32, f32),
    pub color: f32,
}
