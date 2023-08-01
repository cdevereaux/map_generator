use std::collections::BTreeSet;
use std::collections::HashMap;

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

#[derive(PartialEq, Debug)]
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

//chebyshev distance
fn distance(p0: (usize, usize), p1: (usize, usize)) -> usize {
    std::cmp::max(p0.0.abs_diff(p1.0), p0.1.abs_diff(p1.1))
}



pub struct Map {
    color_grid: Vec<Vec<Color>>,
    rng: ThreadRng,
    pub cavern_count: usize,
    pub max_cavern_dist: usize,
    pub walk_count: usize,
    pub walk_len: usize,
}

impl Map {
    const HEIGHT: usize = 1000;
    const WIDTH: usize = 2000;

    pub fn new() -> Self {
        Map {
            color_grid: vec![vec![Color::BLACK; Self::WIDTH]; Self::HEIGHT],
            rng: rand::thread_rng(),
            cavern_count: 12,
            max_cavern_dist: 100,
            walk_count: 50,
            walk_len: 50,
        }
    }

    pub fn reset(&mut self) {
        self.color_grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|p| {
                *p = Color::BLACK;
            })
        });
    }

    //A* search
    fn get_path(&self, start: (usize, usize), target: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let mut open_set = BTreeSet::new(); //(weight, point, came_from)
        let mut best_paths = HashMap::new(); //(point: (came_from, length))
        
        open_set.insert((distance(start, target), start, start));
        while let Some((weight, point, came_from)) = open_set.pop_first() {
            if point == target {
                let mut last_point = target;
                let mut path: Vec<(usize, usize)> = (0..).map_while(|_| {
                    if let Some((next, _)) = best_paths.get(&last_point) {
                        let temp = last_point;
                        last_point = *next;
                        Some(temp)
                    } else {None}
                }).collect();
                path.reverse();
                return Some(path);
            }

            open_set.remove(&(weight, point, came_from));
            for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                let length_delta = if dx != 0 && dy != 0 {2} else {1};
                let tentative_length = weight + length_delta - distance(point, target);

                let next_point = (point.0.saturating_add_signed(dx), point.1.saturating_add_signed(dy));

                if next_point == start {continue;}
                if self.get(next_point.0, next_point.1) == Some(&Color::BLACK) || self.get(next_point.0, next_point.1) == None {continue;}
                
                let successor = (
                    tentative_length + distance(next_point, target),
                    next_point,
                    point
                );

                let updated = if let Some((came_from, length)) = best_paths.get_mut(&next_point) {
                    if *length > tentative_length {
                        *came_from = point;
                        *length = tentative_length;
                        true
                    } else {false}
                } else {
                    best_paths.insert(next_point, (point, tentative_length));
                    true
                };

                if updated {
                    open_set.insert(successor);
                }
                
            }
        }
        None
    }

    
    fn generate_connecting_tunnel(&mut self, start: (usize, usize), target: (usize, usize), color: Color) -> Vec<(usize, usize)> {
        let (mut x, mut y) = start;
        let mut path = Vec::new();
        

        for i in 0.. {
            use CardinalDirection::*;
            let direction_to_target = (
                if target.0.saturating_sub(x) > 0 {Right} else {Left},
                if target.1.saturating_sub(y) > 0 {Up} else {Down},
            );

            let mut rerolls = i%2;
            let next_step = loop {
                let tentative_step = self.rng.gen::<CardinalDirection>();
                if tentative_step != direction_to_target.0 && tentative_step != direction_to_target.1 && rerolls > 0 {
                    rerolls -= 1;
                    continue;
                }
                break tentative_step;
            };

            match next_step {
                Up => y += 1,
                Down => y = y.saturating_sub(1),
                Left => x = x.saturating_sub(1),
                Right => x += 1,
            }
            x = x.clamp(0, Self::WIDTH - 1);
            y = y.clamp(0, Self::HEIGHT - 1);

            path.push((x, y));
            if let Some(tile) = self.get_mut(x, y) {
                *tile = color;
            }
            
            if i%128 == 0 && self.get_path(start, target).is_some() {break;} 
        }
        path
        
    }



    fn random_walk(&mut self, x0: usize, y0: usize, color: Color) -> Vec<(usize, usize)> {
        let (mut x, mut y) = (x0, y0);
        let mut path = Vec::new();

        for _ in 0..self.walk_len {
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
            if let Some(tile) = self.get_mut(x, y) {
                *tile = color;
            }
        }
        path
    }

    pub fn generate(&mut self) {
        self.generate_caverns();
    }

    pub fn generate_caverns(&mut self) {
        let mut caverns = vec![(Self::WIDTH / 2, Self::HEIGHT / 2)];

        while caverns.len() < self.cavern_count {
            let (x, y) = (
                self.rng.gen_range(0..Self::WIDTH),
                self.rng.gen_range(0..Self::HEIGHT),
            );
            if caverns.iter().any(|(x0, y0)| {
                distance((*x0, *y0), (x, y)) < self.max_cavern_dist
            }) {
                caverns.push((x, y));
            }
        }

        let mut first = true; //Temp
        for (x0, y0) in &caverns {
            let mut paths = Vec::new();
            for _ in 0..self.walk_count {
                paths.append(&mut self.random_walk(*x0, *y0, if first {Color::GREEN} else {Color::WHITE}));
            }
            first = false;
        }

        let origin = caverns[0];
        caverns.iter().for_each(|cavern| {
            if self.get_path(origin, *cavern).is_none() {
                
                let closest_unconnected = caverns.iter().filter(|other_cavern| {
                    self.get_path(*cavern, **other_cavern).is_none()
                }).min_by_key(|other_cavern| {
                    distance(*cavern,** other_cavern)
                });

                self.generate_connecting_tunnel(*cavern, *closest_unconnected.unwrap(), Color::WHITE);
            }
        });
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

    fn get(&self, x: usize, y: usize) -> Option<&Color> {
        if let Some(row) = self.color_grid.get(y) {
            row.get(x)
        } else {None}
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        if let Some(row) = self.color_grid.get_mut(y) {
            row.get_mut(x)
        } else {None}
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}
