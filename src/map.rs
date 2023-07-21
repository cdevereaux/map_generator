use eframe::epaint::Color32 as Color;
use rand::Rng;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::rngs::ThreadRng;

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
}

impl Map {
    pub fn new() -> Self {
        Map { color_grid: vec![vec![Color::BLACK; 64]; 64] }
    }

    pub fn reset(&mut self) {
        self.color_grid.iter_mut().for_each(
            |row| {
                row.iter_mut().for_each(|p| {
                    *p = Color::BLACK;
                })
            });
    }

    pub fn random_walk(&mut self, rng: &mut ThreadRng) {
        let height = self.color_grid.len();
        let width = self.color_grid[0].len();
        let mut y = height / 2;
        let mut x = width / 2;

        for _ in 0..100 {
            use CardinalDirection::*;
            match rng.gen::<CardinalDirection>() {
                Up => y += 1,
                Down => y -= 1,
                Left => x -= 1,
                Right => x += 1,
            }
            x = x.clamp(0, width);
            y = y.clamp(0, height);

            self.color_grid[y][x] = Color::WHITE;
        }
    }

    pub fn rows(&self) -> std::ops::Range<usize> {
        0..self.color_grid.len()
    }

    pub fn cols(&self) -> std::ops::Range<usize> {
        0..self.color_grid[0].len()
    }

    pub fn at(&self, x: usize, y: usize) -> Option<Color> {
        if let Some(row) = self.color_grid.get(y) {
            row.get(x).copied()
        }
        else {
            None
        }
    }

}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}