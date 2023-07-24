use eframe::epaint::Color32 as Color;
use eframe::epaint::ColorImage;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::rngs::ThreadRng;
use rand::Rng;

//A list of visually distinct colours
const COLOR_LIST: [Color; 12] = [
    Color::from_rgb(0xa6, 0xce, 0xe3),
    Color::from_rgb(0x1f, 0x78, 0xb4),
    Color::from_rgb(0xb2, 0xdf, 0x8a),
    Color::from_rgb(0x33, 0xa0, 0x2c),
    Color::from_rgb(0xfb, 0x9a, 0x99),
    Color::from_rgb(0xe3, 0x1a, 0x1c),
    Color::from_rgb(0xfd, 0xbf, 0x6f),
    Color::from_rgb(0xff, 0x7f, 0x00),
    Color::from_rgb(0xca, 0xb2, 0xd6),
    Color::from_rgb(0x6a, 0x3d, 0x9a),
    Color::from_rgb(0xff, 0xff, 0x99),
    Color::from_rgb(0xb1, 0x59, 0x28),
];

pub enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<CardinalDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardinalDirection {
        match rng.gen_range(0..4) {
            0 => CardinalDirection::Up,
            1 => CardinalDirection::Down,
            2 => CardinalDirection::Left,
            _ => CardinalDirection::Right,
        }
    }
}

pub struct Map {
    color_grid: Vec<Vec<Color>>,
    rng: ThreadRng,
}

impl Map {
    const HEIGHT: usize = 1000;
    const WIDTH: usize = 2000;

    pub fn new() -> Self {
        Map {
            color_grid: vec![vec![Color::BLACK; Self::WIDTH]; Self::HEIGHT],
            rng: rand::thread_rng(),
        }
    }

    pub fn reset(&mut self) {
        self.color_grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|p| {
                *p = Color::BLACK;
            })
        });
    }

    fn random_walk(&mut self, x0: usize, y0: usize, color: Color) -> Vec<(usize, usize)> {
        let (mut x, mut y) = (x0, y0);
        let mut path = Vec::new();

        for _ in 0..500 {
            use CardinalDirection::*;
            match self.rng.gen::<CardinalDirection>() {
                Up => y += 1,
                Down => y = y.saturating_sub(1),
                Left => x = x.saturating_sub(1),
                Right => x += 1,
            }
            x = x.clamp(0, Self::WIDTH - 1);
            y = y.clamp(0, Self::HEIGHT - 1);

            path.push((x, y));
            self.color_grid[y][x] = color;
        }
        path
    }

    pub fn generate(&mut self) {
        let (mut x0, mut y0) = (Self::WIDTH / 2, Self::HEIGHT / 2);

        for i in 0..12 {
            let color = COLOR_LIST[i % COLOR_LIST.len()];
            let mut paths = Vec::new();
            for _ in 0..5 {
                paths.append(&mut self.random_walk(x0, y0, color))
            }
            let furthest_point = paths.iter().max_by(|(x1, y1), (x2, y2)| {
                let x0 = x0 as i64;
                let y0 = y0 as i64;
                let x1 = *x1 as i64;
                let y1 = *y1 as i64;
                let x2 = *x2 as i64;
                let y2 = *y2 as i64;
                ((x1 - x0).pow(2) + (y1 - y0).pow(2)).cmp(&((x2 - x0).pow(2) + (y2 - y0).pow(2)))
            });
            (x0, y0) = *furthest_point.unwrap_or(&(x0, y0));
        }
    }

    pub fn to_color_image(&self) -> ColorImage {
        let mut pixels = Vec::new();
        for i in 0..self.color_grid.len() {
            pixels.append(&mut self.color_grid[i].clone());
        }
        ColorImage {
            size: [Self::WIDTH, Self::HEIGHT],
            pixels,
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}
