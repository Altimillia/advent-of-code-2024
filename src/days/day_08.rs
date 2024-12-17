use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use crate::domain::point::Point;

pub fn part_one(input: String) -> impl Display {
    let grid = Grid::parse(input.to_string());
    let anti_nodes = calculate_anti_nodes(grid);
    anti_nodes.len()
}

pub fn part_two(input: String) -> impl Display {
    let grid = Grid::parse(input.to_string());
    let anti_nodes = calculate_anti_nodes_p2(grid);
    anti_nodes.len()
}


fn calculate_anti_nodes(grid: Grid) -> HashSet<Point> {
    let mut anti_node_signal_points:HashSet<Point> = HashSet::new();
    let unique_signals = grid.get_unique_signals();

    for unique_signal in unique_signals {
        let anti_nodes = get_anti_nodes(grid.get_signal_points(&unique_signal));

        for anti_node in anti_nodes {
            if(anti_node.x < 0 || anti_node.y < 0 || anti_node.x >= grid.total_size.x || anti_node.y >= grid.total_size.y)
            {
                continue;
            }
            anti_node_signal_points.insert(anti_node);

        }
    }
    anti_node_signal_points
}

fn calculate_anti_nodes_p2(grid: Grid) -> HashSet<Point> {
    let mut anti_node_signal_points:HashSet<Point> = HashSet::new();
    let unique_signals = grid.get_unique_signals();

    for unique_signal in unique_signals {
        let anti_nodes = get_anti_nodes_p2(grid.get_signal_points(&unique_signal), grid.total_size);

        for anti_node in anti_nodes {
            if(anti_node.x < 0 || anti_node.y < 0 || anti_node.x >= grid.total_size.x || anti_node.y >= grid.total_size.y)
            {
                continue;
            }
            anti_node_signal_points.insert(anti_node);

        }
    }
    anti_node_signal_points
}

fn get_anti_nodes(signals: Vec<Point>) -> Vec<Point> {
    let cloned = &signals.clone();
    let mut anti_nodes: Vec<Point> = Vec::new();
    for signal in signals {
        for other_signal in cloned {
            // Draw a line between signal and other signal
            // Take the difference between the two and then double it
            // Take the point that it leads to and add to the list.
            // Move on
            if (signal == *other_signal) {
                continue;
            }

            let vector = signal - *other_signal;
            anti_nodes.push(vector.scale(2) + *other_signal);
        }
    }

    anti_nodes
}

fn get_anti_nodes_p2(signals: Vec<Point>, grid_size:Point) -> Vec<Point> {
    let cloned = &signals.clone();
    let mut anti_nodes: Vec<Point> = Vec::new();
    for signal in signals {
        for other_signal in cloned {
            if (signal == *other_signal) {
                continue;
            }

            let vector = signal - *other_signal;
            let direction = vector.normalize_to_line();
            let mut position = *other_signal;
            loop {
                position = position + direction;
                if position.within_bounds(grid_size, Point::new(0,0)) {
                    anti_nodes.push(position);
                }
                else{
                    break;
                }
            }
        }
    }

    anti_nodes
}


struct Grid {
    grid: HashMap<Point, char>,
    total_size: Point,
}

impl Grid {
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map:HashMap<Point, char> = HashMap::new();
        let mut total_size:Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);

        y_index -= 1;
        for (y, line) in input.lines().enumerate() {
            for (x, node) in line.chars().enumerate() {

                map.insert(Point::new(x as i32, y_index as i32), node);
            }
            y_index = y_index - 1;

        }

        Grid { grid: map, total_size }
    }

    fn get_unique_signals(&self) -> HashSet<char> {
        let mut unique_signals:HashSet<char> = HashSet::new();

        for value in self.grid.values() {
            if(value == &'.') {
                continue;
            }
            unique_signals.insert(*value);
        }

        unique_signals
    }

    fn get_signal_points(&self, signal: &char) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        for (p, v) in self.grid.iter() {
            if (v == signal) {
                points.push(*p);
            }

        }
        points
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_08::{calculate_anti_nodes, calculate_anti_nodes_p2, get_anti_nodes, Grid};
    use crate::domain::point::Point;

    #[test]
    fn can_find_anti_nodes_for_points() {
        let mut nodes:Vec<Point> = Vec::new();

        nodes.push(Point::new(4, 3));
        nodes.push(Point::new(5, 5));

        let anti_nodes = get_anti_nodes(nodes);

        for anti_node in &anti_nodes {
            println!("{}", anti_node);
        }
        assert_eq!(anti_nodes.len(), 2);

    }

    #[test]
    fn can_find_signal_locations() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let grid = Grid::parse(input.to_string());
        let signal_points = grid.get_signal_points(&'A');
        for p in &signal_points {
            println!("{}", p);
        }
        assert_eq!(signal_points.len(),3);
    }

    #[test]
    fn can_calculate_all_valid_anti_nodes() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let grid = Grid::parse(input.to_string());
        let anti_nodes = calculate_anti_nodes(grid);
        assert_eq!(anti_nodes.len(), 14);
    }

    #[test]
    fn can_calculate_all_valid_anti_nodes_with_harmonics() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let grid = Grid::parse(input.to_string());
        let anti_nodes = calculate_anti_nodes_p2(grid);
        assert_eq!(anti_nodes.len(), 34);
    }
}