use eframe::epaint::Color32 as Color;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::rngs::ThreadRng;
use rand::Rng;

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
    origin_x: usize,
    origin_y: usize,
}

impl Map {
    const HEIGHT: usize = 100;
    const WIDTH: usize = 200;

    pub fn new() -> Self {
        Map {
            color_grid: vec![vec![Color::BLACK; Self::WIDTH]; Self::HEIGHT],
            origin_x: Self::WIDTH / 2,
            origin_y: Self::HEIGHT / 2,
        }
    }

    pub fn reset(&mut self) {
        self.color_grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|p| {
                *p = Color::BLACK;
            })
        });
    }

    pub fn random_walk(&mut self, rng: &mut ThreadRng) {
        let mut y = self.origin_y;
        let mut x = self.origin_x;

        for _ in 0..500 {
            use CardinalDirection::*;
            match rng.gen::<CardinalDirection>() {
                Up => y += 1,
                Down => y -= 1,
                Left => x -= 1,
                Right => x += 1,
            }
            x = x.clamp(0, Self::WIDTH);
            y = y.clamp(0, Self::HEIGHT);

            self.color_grid[y][x] = Color::WHITE;
        }
    }

    pub fn height(&self) -> usize {
        Self::HEIGHT
    }

    pub fn width(&self) -> usize {
        Self::WIDTH
    }

    pub fn at(&self, x: usize, y: usize) -> Option<Color> {
        if let Some(row) = self.color_grid.get(y) {
            row.get(x).copied()
        } else {
            None
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}
