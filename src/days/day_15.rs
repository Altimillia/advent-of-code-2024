use std::collections::HashMap;
use std::{fmt, result};
use std::fmt::{write, Display};
use crate::domain::point::{Point, EAST, NORTH, SOUTH, WEST};

pub fn part_one(input: String) -> impl Display {
    let mut split = input.split("\n\n");
    let mut grid = Grid::parse(split.nth(0).unwrap());
    let instructions = parse_instructions(split.nth(0).unwrap());

    grid.print();
    let result = run_simulation(&mut grid, instructions);
    //instructions.iter().for_each(|i| println!("{}", i));
    result
}
pub fn part_two(input: String) -> impl Display {
    // I'm pretty sure I can do this one, but oh boy would it take some rewriting to get there of my Part 1 solution. Might come back to it.
    0
}

fn run_simulation(grid: &mut Grid, instructions: Vec<Point>) -> i32 {
    for instruction in instructions {
        //println!("==========  Instruction {}  ================", instruction);
        let robot_position = grid.get_robot_position();
        //let mut robot_entity = grid.grid.(&robot_position).unwrap();

        let next_position = robot_position + instruction;
        let entity = grid.grid.get_mut(&next_position).unwrap();
        match entity {
            Entity::Empty => {
                // Move the robot
                grid.grid.remove(&next_position);
                grid.grid.insert(next_position, Entity::Robot);
                grid.grid.insert(robot_position, Entity::Empty);
            }
            Entity::Wall => {
                // Do Nothing
            }
            Entity::Box => {
                // Begin the complicated box process
                // First check in that direction, for each cell that has a box, add to the list of boxes to move. If we find an empty space then we allow the move. If a wall we do nothing.
                let mut current_entity = grid.grid.get(&next_position).unwrap();
                let mut current_position = next_position;
                let mut box_position_list:Vec<Point> = Vec::new();


                while matches!(current_entity, Entity::Box) {
                    box_position_list.push(current_position);
                    current_position = current_position + instruction;
                    current_entity = grid.grid.get(&current_position).unwrap();
                }

                match current_entity {
                    Entity::Empty => {
                        // Do the move
                        for box_position in box_position_list.iter().rev() {
                            grid.grid.remove(box_position);
                            grid.grid.insert(*box_position, Entity::Empty);
                            grid.grid.insert(*box_position + instruction, Entity::Box);
                        }

                        grid.grid.remove(&next_position);
                        grid.grid.insert(next_position, Entity::Robot);
                        grid.grid.insert(robot_position, Entity::Empty);

                    }
                    Entity::Wall => {
                        // Do Nothing
                    }
                    Entity::Box => {}
                    Entity::Robot => {}
                }

                // grid.grid.remove(&next_position);
                // grid.grid.insert(next_position, Entity::Robot);
                // grid.grid.insert(robot_position, Entity::Empty);
            }
            Entity::Robot => {
                // This shouldnt be possible
            }
        }

    }
    grid.print();
    grid.get_gps_score()
}

fn parse_instructions(input: &str) -> Vec<Point> {
    input.chars().map(|c| {
        match c {
            '<' => WEST,
            '>' => EAST,
            '^' => SOUTH,
            'v' => NORTH,
            _ => Point::new(0,0)
        }
    }).collect()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Entity {
    Empty,
    Wall,
    Box,
    Robot
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entity::Box => write!(f, "O"),
            Entity::Wall => write!(f, "#"),
            Entity::Empty => write!(f, "."),
            Entity::Robot => write!(f, "@")
        }
    }
}

struct Grid {
    grid: HashMap<Point, Entity>,
    size: Point
}
impl Grid {
    fn parse(input: &str) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map: HashMap<Point, Entity> = HashMap::new();
        let mut total_size: Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);

        y_index -= 1;
        for (y, line) in input.lines().enumerate() {
            for (x, node) in line.chars().enumerate() {
                let entity = match node {
                    '#' => Entity::Wall,
                    'O' => Entity::Box,
                    '@' => Entity::Robot,
                    _ => Entity::Empty
                };

                map.insert(Point::new(x as i32, y as i32), entity);
            }
            y_index = y_index - 1;

        }

        Grid { grid: map, size: total_size }
    }

    fn print(&self) {

        for y in (0..self.size.y) {
            for x in 0..self.size.x {
                print!("{}", self.grid.get(&Point::new(x,y)).unwrap());
            }
            println!("");
        }
    }

    fn get_robot_position(&self) -> Point {
        for kvp in self.grid.iter() {
            if matches!(kvp.1, Entity::Robot) {
                return *kvp.0;
            }
        }
        Point::new(0,0)
    }

    fn get_gps_score(&self) -> i32 {
        self.grid.iter().map(|kvp| {
            if matches!(kvp.1, Entity::Box) {
                return kvp.0.y * 100 + kvp.0.x;
            }
            return 0;
        }).sum::<i32>()
    }
}
#[cfg(test)]
mod tests {
    use crate::days::day_15::{parse_instructions, run_simulation, Grid};

    #[test]
    fn can_run_simulation_on_grid() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

        let mut split = input.split("\n\n");
        let mut grid = Grid::parse(split.nth(0).unwrap());
        let instructions = parse_instructions(split.nth(0).unwrap());
        let result = run_simulation(&mut grid, instructions.clone());

        assert_eq!(result, 2028);
    }
}