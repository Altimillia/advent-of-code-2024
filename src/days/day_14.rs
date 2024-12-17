use std::collections::HashSet;
use std::fmt::Display;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::sequence::{delimited, separated_pair};
use crate::domain::point::Point;
use crate::tools::{parse_numbers_i32, usize_to_i32};

pub fn part_one(input: String) -> impl Display {
    let robots:Vec<Robot> = input.lines().map(|l| Robot::parse(l).unwrap().1).collect();
    let grid = Grid { size: Point::new(101, 103)};

    let simulated = run_simulation(robots, &grid, 300);

    let safety = calculate_safety_factor(simulated, &grid);
    safety
}

pub fn part_two(input: String) -> impl Display {
    let robots:Vec<Robot> = input.lines().map(|l| Robot::parse(l).unwrap().1).collect();
    let grid = Grid { size: Point::new(101, 103)};
    let simulated = run_simulation_with_visual(robots, &grid, 8300);
    0
}

fn run_simulation(robots: Vec<Robot>, grid: &Grid, ticks: i32) -> Vec<Robot> {
    let mut robot_collection = robots;
    for x in 0..ticks {
        robot_collection = robot_collection.iter().map(|r| r.tick_robot(&grid)).collect()
    }

    robot_collection
}

fn run_simulation_with_visual(robots: Vec<Robot>, grid: &Grid, ticks: i32) -> Vec<Robot> {
    let mut robot_collection = robots;
    let left_half = (Point::new(0,0), Point::new(grid.size.x / 2 - 1, grid.size.y - 1));
    let right_half = (Point::new(grid.size.x / 2 + 1, 0), Point::new(grid.size.x - 1, grid.size.y - 1));
    let quadrants = grid.get_quadrants();
    for x in 0..ticks {
        robot_collection = robot_collection.iter().map(|r| r.tick_robot(&grid)).collect();
        //let mid_robots = &robot_collection.iter().filter(|r| r.position.x == grid.size.x / 2).count();
        if(check_if_symmetrical(&robot_collection, grid, &quadrants)){
            display_visual(&robot_collection, grid, x);
        }
    }

    robot_collection
}

fn check_if_symmetrical(robots: &Vec<Robot>, grid: &Grid, quadrants: &Vec<(Point, Point)>) -> bool {
    let unique_positions:HashSet<Point> = robots.iter().map(|x| x.position).collect();
    let top_left = quadrants.iter().nth(0).unwrap();
    let top_right = quadrants.iter().nth(1).unwrap();
    let bottom_left = quadrants.iter().nth(2).unwrap();
    let bottom_right = quadrants.iter().nth(3).unwrap();



    let top_left_count = unique_positions.iter().filter(|r| r.within_bounds_inclusive(top_left.1, top_left.0)).count();
    let top_right_count = unique_positions.iter().filter(|r|  r.within_bounds_inclusive(top_right.1, top_right.0)).count();
    let bottom_left_count = unique_positions.iter().filter(|r| r.within_bounds_inclusive(bottom_left.1, bottom_left.0)).count();
    let bottom_right_count = unique_positions.iter().filter(|r| r.within_bounds_inclusive(bottom_right.1, bottom_right.0)).count();

    if(bottom_left_count > 300){
        println!("{} - {} - {} - {}", top_left_count, top_right_count, bottom_left_count, bottom_right_count);
        return true;
    }
    if(unique_positions.len() == robots.len()){

        println!("{} - {}", unique_positions.len(), robots.len());
        println!("{} - {} - {} - {}", top_left_count, top_right_count, bottom_left_count, bottom_right_count);
        return true;
    }


    if(top_left_count != top_right_count)
    {
        return false;
    }

    if(bottom_left_count != bottom_right_count) {
        return false;
    }

    false
}
fn check_if_symmetrical_2(robots: &Vec<Robot>, grid: &Grid, left_half: &(Point, Point), right_half: &(Point, Point)) -> bool {
    let left_count = robots.iter().filter(|r| r.is_in_quadrant(*left_half)).count();
    let right_count = robots.iter().filter(|r| r.is_in_quadrant(*right_half)).count();
    if(left_count != right_count)
    {
        return false;
    }

    print!("{} - {}", left_count, right_count);

    true
}

fn display_visual(robots: &Vec<Robot>, grid: &Grid, current_tick: i32) {
    println!("Current Tick {}", current_tick);
    for y in (0..grid.size.y) {
        for x in 0..grid.size.x {
            let p = &Point::new(x,y);
            let robots = robots.iter().filter(|x| x.position == *p).count();
            if(robots > 0){
                print!("{}", robots);
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input_line, (left,right)) = separated_pair(parse_numbers_i32, tag(","), parse_numbers_i32)(input)?;
    Ok((input_line, Point::new(left,right)))
}

fn calculate_safety_factor(robots: Vec<Robot>, grid: &Grid) -> i32 {
    // get quadrant sections
    let quadrants = grid.get_quadrants();
    // for robot in &robots {
    //     println!("{}", robot.position)
    // }
    let mut section_counts:Vec<usize> = quadrants.iter()
        .map(|quad|{
            let count = robots.iter()
                .filter(|r| r.position.within_bounds_inclusive(quad.1, quad.0)).count();
            // println!("{} {} {}", quad.0, quad.1, count);
            return count;
        })
        .collect();

    // for section_count in section_counts {
    //     println!("{}", section_count);
    // }
    //
    usize_to_i32(section_counts.iter().nth(0).unwrap()
        * section_counts.iter().nth(1).unwrap()
        * section_counts.iter().nth(2).unwrap()
        * section_counts.iter().nth(3).unwrap()).unwrap()

}

struct Robot {
    position: Point,
    velocity: Point
}
impl Robot {
    fn parse(input: &str) -> IResult<&str, Robot> {
        let (input, _) = tag("p=")(input)?;
        let (input, position) = parse_point(input)?;
        let (input, _) = tag(" v=")(input)?;
        let (input, velocity) = parse_point(input)?;

        Ok((input, Robot {position, velocity }))
    }

    fn tick_robot(&self, grid: &Grid) -> Robot {
        let mut next_position = self.position + self.velocity;
        if (next_position.x >= grid.size.x) {
            next_position = Point::new(next_position.x - grid.size.x, next_position.y);
        }
        else if(next_position.x < 0) {
            // then we do the opposite here
            next_position = Point::new(next_position.x + grid.size.x, next_position.y);
        }

        if (next_position.y >= grid.size.y) {

            next_position = Point::new(next_position.x, next_position.y - grid.size.y);
        }
        else if(next_position.y < 0) {
            // then we do the opposite here
            next_position = Point::new(next_position.x, next_position.y + grid.size.y);
        }

        Robot { position:next_position, velocity: self.velocity }
    }

    fn is_in_quadrant(&self, quad: (Point, Point)) -> bool {
        self.position.within_bounds_inclusive(quad.1, quad.0)
    }

}

struct Grid {
    size: Point
}

impl Grid {
    fn get_quadrants(&self) -> Vec<(Point, Point)> {
        let top_left = (Point::new(0,0), Point::new(self.size.x / 2 - 1, self.size.y / 2 - 1));
        let top_right = (Point::new(self.size.x / 2 + 1, 0), Point::new(self.size.x - 1, self.size.y / 2 - 1));
        let bottom_left = (Point::new(0, self.size.y / 2 + 1), Point::new(self.size.x / 2 - 1, self.size.y - 1));
        let bottom_right = (Point::new(self.size.x / 2 + 1, self.size.y / 2 + 1), Point::new(self.size.x - 1, self.size.y - 1));

        vec![top_left, top_right, bottom_left, bottom_right]
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_14::{calculate_safety_factor, run_simulation, Grid, Robot};
    use crate::domain::point::Point;

    #[test]
    fn robot_can_teleport_when_moving() {
        let input = r#"p=2,4 v=2,-3"#;

        let robot = Robot::parse(input).unwrap().1;
        let grid = Grid { size: Point::new(11, 7)};

        let next = robot.tick_robot(&grid).tick_robot(&grid);

        assert_eq!(next.position, Point::new(6,5))
    }

    #[test]
    fn robot_simulations_can_determine_location() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

        let robots:Vec<Robot> = input.lines().map(|l| Robot::parse(l).unwrap().1).collect();
        let grid = Grid { size: Point::new(11, 7)};

        let simulated = run_simulation(robots, &grid, 100);

        let safety = calculate_safety_factor(simulated, &grid);

        assert_eq!(safety, 12);
    }

    #[test]
    fn get_grid_quadrants() {
        let grid = Grid { size: Point::new(11, 7)};

        let mut quadrants = grid.get_quadrants();
        assert_eq!(quadrants.len(), 4);
        assert_eq!(*quadrants.iter().nth(0).unwrap(), (Point::new(0,0), Point::new(4,2)));
        assert_eq!(*quadrants.iter().nth(1).unwrap(), (Point::new(6,0), Point::new(10,2)));
        assert_eq!(*quadrants.iter().nth(2).unwrap(), (Point::new(0,4), Point::new(4,6)));
        assert_eq!(*quadrants.iter().nth(3).unwrap(), (Point::new(6,4), Point::new(10,6)));
    }
}