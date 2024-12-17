use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use crate::domain::point::{Point, NORTH, SOUTH, EAST, WEST};

pub fn part_one(input: String) -> impl Display {
    let mut grid = Grid::parse(input);
    let guard = Guard { position: grid.current_guard_position, direction: NORTH };
    let traveled = walk_the_grid(&grid, guard);
    grid.print_traveled(traveled.clone());

    traveled.len()
}
pub fn part_two(input: String) -> impl Display {
    let mut grid = Grid::parse(input.to_string());
    let guard = Guard { position: grid.current_guard_position, direction: NORTH };
    let traveled = walk_the_grid(&grid, guard.clone());

    let obstacles = brute_force_put_obstacles(guard.clone(), grid, traveled);
    obstacles.len()
}

fn walk_the_grid(grid: &Grid, guard: Guard) -> HashSet<Point> {

    // check next position
    // If obstacle, turn 90 degrees
    // If not, move forward.
    let mut guard_check = guard.clone();
    let mut traveled:HashSet<Point> = HashSet::new();

    traveled.insert(guard_check.position);

    loop {
        let next_point = guard_check.position + guard_check.direction;
        if !grid.grid.contains_key(&next_point) {
            break;
        }
        let current_entity = grid.grid.get(&next_point).unwrap();
        match current_entity {
            Entity::Obstacle => {
                // Rotate direction
                guard_check = Guard {direction: get_rotation(guard_check.direction), position: guard_check.position };
            }
            Entity::Empty => {
                // Move forward
                traveled.insert(next_point);
                guard_check = Guard {direction: guard_check.direction, position: next_point };
            }
        }
        //print!("{}", next_point);
        if(guard_check.position.x < 0 || guard_check.position.y < 0 || guard_check.position.x >= grid.total_size.x || guard_check.position.y >= grid.total_size.y)
        {
            break;
        }
    }


    traveled
}


fn check_if_looped(grid: &Grid, guard: Guard, max_iterations: i32) -> bool {

    // check next position
    // If obstacle, turn 90 degrees
    // If not, move forward.
    let mut current_iterations = 0;
    let mut guard_check = guard.clone();
    let mut traveled:HashSet<Point> = HashSet::new();

    traveled.insert(guard_check.position);

    loop {
        let next_point = guard_check.position + guard_check.direction;
        if !grid.grid.contains_key(&next_point) {
            return false;
        }
        let current_entity = grid.grid.get(&next_point).unwrap();
        match current_entity {
            Entity::Obstacle => {
                // Rotate direction
                guard_check = Guard {direction: get_rotation(guard_check.direction), position: guard_check.position };
            }
            Entity::Empty => {
                // Move forward
                traveled.insert(next_point);
                guard_check = Guard {direction: guard_check.direction, position: next_point };
            }
        }

        if(guard_check.position.x < 0 || guard_check.position.y < 0 || guard_check.position.x >= grid.total_size.x || guard_check.position.y >= grid.total_size.y)
        {
            return false;
        }
        current_iterations += 1;

        if(current_iterations > max_iterations) {
            return true;
        }
    }


    return false;
}



fn brute_force_put_obstacles(guard: Guard, mut grid: Grid, traveled: HashSet<Point>) -> Vec<Point> {
    let mut obstacle_positions:Vec<Point> = Vec::new();
    for point in traveled {
        // Do Inner Loop Check
        let mut updated_grid = grid.clone();
        updated_grid.grid.insert(point, Entity::Obstacle);
        if check_if_looped(&updated_grid, guard.clone(), 10000) {
            obstacle_positions.push(point);
        }
    }

    obstacle_positions

}

fn get_rotation(direction: Point) -> Point {
    if(direction == NORTH) {
        return EAST
    }
    if(direction == EAST) {
        return SOUTH;
    }
    if(direction == SOUTH) {
        return WEST;
    }
    if(direction == WEST) {
        return NORTH
    }

    return direction
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Entity {
    Obstacle,
    Empty
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entity::Obstacle => write!(f, "#"),
            Entity::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Guard {
    position: Point,
    direction: Point
}

#[derive(PartialEq, Eq, Clone)]
struct Grid {
    grid: HashMap<Point, Entity>,
    total_size: Point,
    current_guard_position: Point
}

impl Grid {
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map:HashMap<Point, Entity> = HashMap::new();
        let mut total_size:Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);
        let mut current_guard_position = Point::new(0,0);
        y_index -= 1;
        for (y, line) in input.lines().enumerate() {
            for (x, node) in line.chars().enumerate() {
                let entity = match node {
                    '#' => Entity::Obstacle,
                    _ => Entity::Empty
                };

                if(node == '^') {
                    current_guard_position = Point::new(x as i32, y_index as i32);
                }

                map.insert(Point::new(x as i32, y_index as i32), entity);
            }
            y_index = y_index - 1;

        }

        Grid { grid: map, total_size, current_guard_position }
    }

    fn print(&self) {

        for y in (0..self.total_size.y).rev() {
            for x in 0..self.total_size.x {
                print!("{}", self.grid.get(&Point::new(x,y)).unwrap());
            }
            println!("");
        }
    }

    fn print_traveled(&self, traveled: HashSet<Point>) {
        for y in (0..self.total_size.y).rev() {
            for x in 0..self.total_size.x {
                let p = &Point::new(x,y);
                if(traveled.contains(p)){
                    print!("{}", "X");
                }
                else {
                    print!("{}", self.grid.get(&Point::new(x, y)).unwrap());
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::point::NORTH;
    use super::{brute_force_put_obstacles, walk_the_grid, Grid, Guard};

    #[test]
    fn can_track_guard_positions(){
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let mut grid = Grid::parse(input.to_string());
        let guard = Guard { position: grid.current_guard_position, direction: NORTH };
        let traveled = walk_the_grid(&grid, guard);

        grid.print_traveled(traveled.clone());

        assert_eq!(traveled.len(), 41);
    }

    #[test]
    fn can_track_loop_positions(){
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let mut grid = Grid::parse(input.to_string());
        let guard = Guard { position: grid.current_guard_position, direction: NORTH };
        let traveled = walk_the_grid(&grid, guard.clone());

        grid.print_traveled(traveled.clone());
        let obstacles = brute_force_put_obstacles(guard.clone(), grid, traveled);

        assert_eq!(obstacles.len(), 6);
    }
}