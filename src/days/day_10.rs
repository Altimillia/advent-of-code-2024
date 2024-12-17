use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use itertools::all;
use crate::domain::point::{Point, EAST, NORTH, SOUTH, WEST};
use crate::tools::parse_numbers_i32;

pub fn part_one(input: String) -> impl Display {
    let grid = Grid::parse(input.to_string());
    let result = get_trail_head_scores(&grid);
    result
}

pub fn part_two(input: String) -> impl Display {
    let grid = Grid::parse(input.to_string());
    get_trail_head_ratings(&grid)
}

fn get_trail_head_scores(grid: &Grid) -> usize {
    let mut total_score = 0;
    let trail_heads = grid.trail_heads.clone();
    for trail_head in trail_heads {
        let paths = recursive_path_finding(PathFit { path: vec![trail_head]}, grid, trail_head);
        let endpoints:HashSet<Point> = paths.iter().map(|p| *p.path.last().unwrap()).collect();
        total_score += endpoints.len();
    }

    total_score
}

fn get_trail_head_ratings(grid: &Grid) -> usize {
    let mut total_score = 0;
    let trail_heads = grid.trail_heads.clone();
    for trail_head in trail_heads {
        let paths = recursive_path_finding(PathFit { path: vec![trail_head]}, grid, trail_head);
        total_score += paths.len();
    }

    total_score
}


fn recursive_path_finding(path_fit: PathFit, grid: &Grid, current_point: Point) -> Vec<PathFit> {
    let neighbors = grid.get_neighbors(current_point);
    let mut paths:Vec<PathFit> = Vec::new();
    let current_level = grid.grid.get(&current_point).unwrap();
    if current_level == &9 {
        return vec![path_fit]
    }

    for neighbor in neighbors {
        let next_point = grid.grid.get(&neighbor).unwrap();
        if(*next_point != *current_level + 1){
            continue;
        }
        let mut branch = path_fit.clone();
        branch.path.push(neighbor);

        paths.extend(recursive_path_finding(branch, grid, neighbor));
    }

    paths
}


struct Grid {
    grid: HashMap<Point, u32>,
    total_size: Point,
    trail_heads: Vec<Point>
}

impl Grid {
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map: HashMap<Point, u32> = HashMap::new();
        let mut total_size: Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);
        let mut trail_heads:Vec<Point> = Vec::new();
        y_index -= 1;
        for (y, line) in input.lines().enumerate() {
            for (x, node) in line.chars().enumerate() {
                let digit = node.to_digit(10).unwrap();
                if(digit == 0){
                    trail_heads.push(Point::new(x as i32, y_index as i32));
                }
                map.insert(Point::new(x as i32, y_index as i32), digit);
            }
            y_index = y_index - 1;
        }

        Grid { grid: map, total_size, trail_heads }
    }

    fn get_neighbors(&self, pos: Point) -> Vec<Point> {
        let neighbors = pos.get_cardinal_neighbors();
        let mut valid:Vec<Point> = Vec::new();
        for neighbor in neighbors {
            if(neighbor.within_bounds(self.total_size, Point::new(0,0))){
                valid.push(neighbor);
            }
        }

        valid
    }
}

#[derive(Debug, Clone)]
struct PathFit {
    path: Vec<Point>
}

#[cfg(test)]
mod tests {
    use crate::days::day_10::{get_trail_head_scores, Grid};

    #[test]
    fn can_calculate_trail_head_scores(){
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        let grid = Grid::parse(input.to_string());
        let score = get_trail_head_scores(&grid);

        assert_eq!(score, 36);
    }
}