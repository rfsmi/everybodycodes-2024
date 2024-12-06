use std::collections::{HashMap, HashSet};

use itertools::iproduct;

struct Area {
    diggable: bool,
    depth: usize,
}

struct State {
    grid: HashMap<(isize, isize), Area>,
    height: isize,
    width: isize,
    neighbours: Vec<(isize, isize)>,
}

impl State {
    fn parse(input: &str) -> Self {
        let mut grid = HashMap::new();
        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                let diggable = match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!(),
                };
                grid.insert((y as isize, x as isize), Area { diggable, depth: 0 });
            }
        }
        let height = grid.keys().map(|(y, _)| y + 1).max().unwrap();
        let width = grid.keys().map(|(_, x)| x + 1).max().unwrap();
        Self {
            grid,
            height,
            width,
            neighbours: [(0, 1), (0, -1), (1, 0), (-1, 0)].into(),
        }
    }

    fn diagonals(self) -> Self {
        Self {
            neighbours: iproduct!([-1, 0, 1], [-1, 0, 1]).collect(),
            ..self
        }
    }

    fn dig(&mut self, (y, x): (isize, isize)) -> bool {
        let depth = match self.grid.get(&(y, x)) {
            Some(&Area { diggable, depth }) if diggable => depth + 1,
            _ => return false,
        };
        for (dy, dx) in self.neighbours.iter() {
            let pos = (y + dy, x + dx);
            let neighbour_depth = self.grid.get(&pos).map_or(0, |area| area.depth);
            if neighbour_depth.abs_diff(depth) > 1 {
                return false;
            }
        }
        self.grid.entry((y, x)).and_modify(|area| area.depth += 1);
        true
    }

    fn solve(mut self) -> usize {
        let mut total_digs = 0;
        loop {
            let mut any_digs = false;
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.dig((y, x)) {
                        any_digs = true;
                        total_digs += 1;
                    }
                }
            }
            if !any_digs {
                break;
            }
        }
        total_digs
    }
}

pub fn solve_1(input: &str) -> usize {
    State::parse(input).solve()
}

pub fn solve_2(input: &str) -> usize {
    State::parse(input).solve()
}

pub fn solve_3(input: &str) -> usize {
    State::parse(input).diagonals().solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
        ..........
        ..###.##..
        ...####...
        ..######..
        ..######..
        ...####...
        ..........
    ";

    #[test]
    fn test_1() {
        assert_eq!(solve_1(SAMPLE), 35);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve_3(SAMPLE), 29);
    }
}
