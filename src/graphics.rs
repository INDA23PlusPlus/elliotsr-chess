#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            r,
            g,
            b
        }
    }

    fn to_bg_string(&self) -> String {
        format!("\x1B[48;2;{};{};{}m", self.r, self.g, self.b)
    }

    fn to_fg_string(&self) -> String {
        format!("\x1B[38;2;{};{};{}m", self.r, self.g, self.b)
    }

}
    
pub struct Screen {
    width: usize,
    height: usize,
    foreground: Vec<Vec<Color>>,
    background: Vec<Vec<Color>>,
    sprites: Vec<Vec<char>>
}

impl Screen {

    pub fn new(width: usize, height: usize) -> Self {
        Screen {
            width,
            height,
            background: vec![vec![Color::new(0, 0, 0); width]; height],
            foreground: vec![vec![Color::new(255, 255, 255); width]; height],
            sprites: vec![vec![' '; width]; height],
        }
    }

    pub fn clear(&mut self, background: Option<Color>, foreground: Option<Color>, sprite: Option<char>) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(bg) = background {
                    self.background[y][x] = bg;
                }
                if let Some(fg) = foreground {
                    self.foreground[y][x] = fg;
                }
                if let Some(c) = sprite {
                    self.sprites[y][x] = c;
                }
            }
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> (Color, Color, char) {
        (self.background[y][x], self.foreground[y][x], self.sprites[y][x])
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, background: Option<Color>, foreground: Option<Color>, sprite: Option<char>) -> () {
        if let Some(bg) = background {
            self.background[y][x] = bg;
        }
        if let Some(fg) = foreground {
            self.foreground[y][x] = fg;
        }
        if let Some(c) = sprite {
            self.sprites[y][x] = c;
        }
    }

    pub fn render(&self, flip_x: bool , flip_y: bool) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let x = if flip_x { self.width - 1 - x } else { x };
                let y = if flip_y { self.height - 1 - y } else { y };
                let bg = self.background[y][x].to_bg_string();
                let fg = self.foreground[y][x].to_fg_string();
                let c = self.sprites[y][x];
                result.push_str(format!("{}{} {} ", bg, fg, c).as_str());
            }
            result.push('\n');
        }
        result.push_str("\x1B[0m");
        result
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}