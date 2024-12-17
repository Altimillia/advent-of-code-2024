use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Display;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::domain::point::{Point, EAST};

pub fn part_one(input: String) -> impl Display {
    let grid = Grid::parse(input.clone());

    let result = non_recursive_path_finding(PathFit { path: vec![grid.start], cost: 0 }, &grid, grid.start, EAST);
    result.iter().map(|x| x.cost).min().unwrap();
}

pub fn part_two(input: String) -> impl Display {
    let grid = Grid::parse(input.clone());
    let result = non_recursive_path_finding(PathFit { path: vec![grid.start], cost: 0 }, &grid, grid.start, EAST);
    let mut point_hash:HashSet<Point> = HashSet::new();

    for path in result {
        println!("{} {}", path.path.len(), path.cost);
        if(path.cost == 101492){
            for p in path.path.clone() {
                point_hash.insert(p);
            }
        }
    }
    point_hash.len()
}

fn a_star_path_finding(grid: &Grid, start_point: Point) -> i32 {
    let mut frontier: PriorityQueue<Point, i32> = PriorityQueue::new();
    let mut closed: Vec<(Point, Point)> = Vec::new();
    let mut cost_so_far: HashMap<Point, i32> = HashMap::new();
    let mut came_from = HashMap::new();

    frontier.push(start_point, 0);

    came_from.insert(start_point, None);
    cost_so_far.insert(start_point, 0);

    while frontier.len() > 0 {
        let popped = frontier.pop().unwrap();


        let current_pos = popped.0;
        if current_pos == grid.end {
            break;
        }

        let mut direction = Point::new(0,0);
        let came_from_position = came_from.get(&current_pos).unwrap();
        // get the direction, use that to determine rotations
        if came_from_position.is_some()  {
            direction = current_pos - came_from_position.unwrap();
        }
        else {
            direction = EAST;
        }

        if(closed.contains(&(current_pos, direction))){
            continue;
        }

        closed.push((current_pos, direction));


        let neighbors = grid.get_neighbors(current_pos);

        for neighbor in neighbors {

            // What should have the rotation? We want it calculated based on where were coming from.

            let mut cost = *cost_so_far.get(&current_pos).unwrap();
            if(neighbor == current_pos + direction){
                cost += 1;
            }
            else if(neighbor == current_pos - direction) {
                continue;
            }
            else{
                cost += 1001;
            }


            // if frontier.get(&neighbor).is_some() && cost > *cost_so_far.get(&neighbor).unwrap() {
            //     frontier.remove(&neighbor);
            // }

            // if closed.contains(&neighbor) && cost < *cost_so_far.get(&neighbor).unwrap()  {
            //     closed.retain(|&x| x != neighbor);
            // }

            if !frontier.get(&neighbor).is_some() //&& !closed.contains(&neighbor)
            {
                cost_so_far.insert(neighbor, cost);
                let priority = cost;// heuristic(neighbor, grid.end);
                frontier.push(neighbor, priority * -1);
                came_from.insert(neighbor, Some(current_pos));
            }
        }
    }
    let mut path = Vec::new();
    let mut current = grid.end;
    while let Some(&Some(prev)) = came_from.get(&current) {
        path.push(current);
        current = prev;
    }
    let total_cost = cost_so_far.get(&grid.end).unwrap();
    println!("{}", total_cost);
    path.push(grid.start);
    path.reverse();

    grid.print_grid(&path);

    *total_cost
}

fn non_recursive_path_finding(mut path_fit: PathFit, grid: &Grid, start_point: Point, direction: Point) -> Vec<PathFit> {
    let mut stack: Vec<(PathFit, Point, Point)> = Vec::new(); // Stack to store state: (PathFit, current_point, direction)
    let mut point_score: HashMap<(Point, Point), i32> = HashMap::new();
    let mut paths: Vec<PathFit> = Vec::new();

    let mut best_cost = 0;
    stack.push((path_fit, start_point, direction));

    while let Some((current_path_fit, current_point, current_direction)) = stack.pop() {
        let neighbors = grid.get_neighbors(current_point);

        if(best_cost != 0 && point_score.contains_key(&(current_point, current_direction)) && *point_score.get(&(current_point, current_direction)).unwrap() < current_path_fit.cost) {
            continue;
        }
        if(best_cost != 0 && current_path_fit.cost > best_cost) {
            continue;
        }

        if current_point == grid.end {
            let current_cost = current_path_fit.cost;
            if(best_cost == 0) {
                best_cost = current_cost;
                println!("New Best: {}", best_cost);
            }
            else if(best_cost >= current_cost) {
                best_cost = current_cost;
                println!("New Best: {}", best_cost);
            }
            else {
                continue;
            }
            paths.push(current_path_fit);
            continue;
        }

        for neighbor in &neighbors {
            if current_path_fit.path.contains(&neighbor) {
                continue;
            }
            if(*neighbor == current_point + current_direction) {
                continue;
            }

            let mut branch = current_path_fit.clone();
            if *neighbor == current_point + current_direction {
                branch.cost += 1;
            } else if *neighbor == current_point - current_direction {
                branch.cost += 2001;
            } else {
                branch.cost += 1001;
            }

            if(point_score.contains_key(&(*neighbor, current_direction)) && branch.cost < *point_score.get(&(*neighbor, current_direction)).unwrap()){
                point_score.remove(&(*neighbor, current_direction));
                point_score.insert((*neighbor, current_direction), branch.cost);
            }
            else if(!point_score.contains_key(&(*neighbor, current_direction))){
                point_score.insert((*neighbor, current_direction), branch.cost);
            }

            branch.path.push(*neighbor);
            stack.push((branch, *neighbor, *neighbor - current_point));
        }
        if(neighbors.contains(&(current_point + current_direction))) {
            let next = (current_point + current_direction);
            if !current_path_fit.path.contains(&next) {
                let mut branch = current_path_fit.clone();
                branch.cost += 1;

                let key = (next, current_direction);
                if(point_score.contains_key(&key) && branch.cost < *point_score.get(&key).unwrap()){
                    point_score.remove(&key);
                    point_score.insert(key, branch.cost);
                }
                else if(!point_score.contains_key(&key)){
                    point_score.insert(key, branch.cost);
                }

                branch.path.push(next);
                stack.push((branch, next, next - current_point));
            }
        }

    }

    paths
}

fn non_recursive_path_finding_2(mut path_fit: PathFit, grid: &Grid, start_point: Point, direction: Point) -> Vec<PathFit> {
    //let mut stack: Vec<(PathFit, Point, Point)> = Vec::new(); // Stack to store state: (PathFit, current_point, direction)
    let mut frontier: PriorityQueue<(PathFit, Point, Point), i32> = PriorityQueue::new();
    let mut paths: Vec<PathFit> = Vec::new();

    let mut best_cost = 0;
    //stack.push((path_fit, start_point, direction));
    frontier.push((path_fit, start_point, direction), 0);

    while let Some(((current_path_fit, current_point, current_direction), cost)) = frontier.pop() {
        let neighbors = grid.get_neighbors(current_point);
        //println!("cost {}", cost);
        if(best_cost != 0 && current_path_fit.cost > best_cost) {
            continue;
        }

        if current_point == grid.end {
            let current_cost = current_path_fit.cost;
            if(best_cost == 0) {
                best_cost = current_cost;
                println!("New Best: {}", best_cost);
            }
            else if(best_cost > current_cost) {
                best_cost = current_cost;
                println!("New Best: {}", best_cost);
            }
            else {
                continue;
            }
            paths.push(current_path_fit);
            continue;
        }

        for neighbor in &neighbors {
            if current_path_fit.path.contains(&neighbor) {
                continue;
            }
            if(*neighbor == current_point + current_direction) {
                continue;
            }

            let mut branch = current_path_fit.clone();
            if *neighbor == current_point + current_direction {
                branch.cost += 1;
            } else if *neighbor == current_point - current_direction {
                branch.cost += 2001;
            } else {
                branch.cost += 1001;
            }

            let cost = branch.cost + heuristic(*neighbor, grid.end);;
            branch.path.push(*neighbor);
            frontier.push((branch, *neighbor, *neighbor - current_point), cost * -1);
        }
        if(neighbors.contains(&(current_point + current_direction))) {
            let next = (current_point + current_direction);
            if !current_path_fit.path.contains(&next) {
                let mut branch = current_path_fit.clone();
                branch.cost += 1;
                branch.path.push(next);
                let cost = branch.cost + heuristic(next, grid.end);;
                frontier.push((branch, next, next - current_point), cost * -1);
            }
        }

    }

    paths
}

fn heuristic(a:Point, b:Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn heuristic_with_rotation(a:Point, b:Point, rotation: Rotation) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

enum Rotation {
    Straight,
    Ninety
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PathFit {
    path: Vec<Point>,
    cost: i32
}

struct Grid {
    nodes: HashMap<Point, char>,
    total_size: Point,
    start: Point,
    end: Point,
}
impl Grid {
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map: HashMap<Point, char> = HashMap::new();
        let mut total_size: Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);

        let mut start = Point::new(0,0);
        let mut end = Point::new(0,0);
        for (y, line) in input.lines().enumerate() {
            for (x, node) in line.chars().enumerate() {

                map.insert(Point::new(x as i32, y as i32), node);
                match node {
                    'S' => { start = Point::new(x as i32, y as i32); }
                    'E' => { end = Point::new(x as i32, y as i32); }
                    _ => {}
                }
            }
            y_index = y_index - 1;
        }

        Grid { nodes: map, total_size, start, end }
    }

    fn get_neighbors(&self, pos: Point) -> Vec<Point> {
        pos.get_cardinal_neighbors().iter()
            .filter(|p| self.nodes.contains_key(p))
            .filter(|p| *self.nodes.get(&(p)).unwrap() != '#').map(|p| *p).collect()

    }

    fn print_grid(&self, movement: &Vec<Point>) {
        for y in (0..self.total_size.y) {
            for x in 0..self.total_size.x {
                if movement.contains(&Point::new(x, y)) {
                    print!("O");
                } else {
                    //print!("{}", self.map.get(&Point::new(x, y)).unwrap().heat_loss);
                    print!("{}", self.nodes.get(&Point::new(x, y)).unwrap());
                }
            }
            println!("");
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::days::day_16::{a_star_path_finding, non_recursive_path_finding, Grid, PathFit};
    use crate::domain::point::EAST;

    #[test]
    fn can_get_path_to_end_with_a_star(){
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

        let grid = Grid::parse(input.to_string());

        let result = a_star_path_finding(&grid, grid.start);
        assert_eq!(result, 7036);
    }

    #[test]
    fn can_get_path_to_end_with_recursive(){
        let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

        let grid = Grid::parse(input.to_string());

        let result = non_recursive_path_finding(PathFit { path: vec![grid.start], cost: 0 }, &grid, grid.start, EAST);

        let min_cost = result.iter().map(|x| x.cost).min();
        assert_eq!(min_cost.unwrap(), 11048);
    }

    #[test]
    fn more_complex_path() {
        let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

        let grid = Grid::parse(input.to_string());

        // let result = non_recursive_path_finding(PathFit { path: vec![grid.start], cost: 0 }, &grid, grid.start, EAST);

        let result = a_star_path_finding(&grid, grid.start);
        assert_eq!(result, 11048);
    }

}