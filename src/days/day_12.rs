use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use itertools::{Itertools, Positions};
use crate::domain::point::{Point, EAST, NORTH, SOUTH, WEST};
use crate::tools::usize_to_i32;

pub fn part_one(input: String) -> impl Display {
    let grid = Grid::parse(input.to_string());
    let regions = get_regions(&grid);
    let result = regions.iter().map(|x| x.get_price()).sum::<i32>();
    result
}

pub fn part_two(input: String) -> impl Display {
    let grid = Grid::parse(input.to_string());
    let regions = get_regions(&grid);
    let result = regions.iter().map(|x| x.get_price_bulk_discount()).sum::<i32>();

    result
}

fn get_regions(grid: &Grid) -> Vec<Region> {
    let mut regions:Vec<Region> = Vec::new();

    for key in grid.grid.keys()
    {
        if(regions.iter().any(|x| x.positions.contains(key))){
            continue;
        }

        regions.push(grid.get_region(*key));
    }

    regions
}

struct Grid {
    grid: HashMap<Point, char>,
    total_size: Point
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

    fn get_region(&self, start_point: Point) -> Region {
        let mut queue = VecDeque::new();
        let plant = self.grid.get(&start_point).unwrap();
        let mut region = Region { positions: vec![], plant: *plant };

        queue.push_back(start_point);
        queue.push_back(start_point + NORTH);
        queue.push_back(start_point+ SOUTH);
        queue.push_back(start_point + EAST);
        queue.push_back(start_point + WEST);

        while let Some(next) = queue.pop_front() {

            if !self.grid.contains_key(&next) {
                continue;
            }
            if plant != self.grid.get(&next).unwrap() {
                continue;
            }
            if(region.positions.contains(&next)){
                continue;
            }

            region.positions.push(next);

            let neighbors = next.get_cardinal_neighbors();
            for neighbor in neighbors {
                queue.push_back(neighbor);
            }
        }

        region
    }
}

struct Region {
    plant: char,
    positions: Vec<Point>
}
impl Region {
    fn get_area(&self) -> usize {
        self.positions.len()
    }

    fn calculate_perimeter(&self) -> i32 {
        let mut perimeter = 0;

        for position in &self.positions {
            let neighbors = position.get_cardinal_neighbors();

            for neighbor in neighbors {
                if(self.positions.contains(&neighbor)){
                    continue;
                }
                perimeter += 1;
            }
        }

        perimeter
    }

    fn get_price(&self) -> i32 {
        usize_to_i32(self.get_area()).unwrap() * self.calculate_perimeter()
    }

    fn get_price_bulk_discount(&self) -> i32 {
        usize_to_i32(self.get_edges().len()).unwrap() * usize_to_i32(self.get_area()).unwrap()
    }

    fn get_edges(&self) -> Vec<CombinedEdge> {
        let mut edge_map:HashMap<(Point,Point), SingleEdge> = HashMap::new();
        for position in &self.positions {
            let neighbors = position.get_cardinal_neighbors();

            for neighbor in neighbors {
                if(self.positions.contains(&neighbor)){
                    continue;
                }
                edge_map.insert((*position, neighbor - *position), SingleEdge { position: *position, direction: neighbor - *position });
            }
        }

        let mut combined_edges:Vec<CombinedEdge> = Vec::new();
        let mut mapped_edges:Vec<(Point,Point)> = Vec::new();

        for key in edge_map.keys() {
            if (mapped_edges.contains(key)) {
                continue;
            }

            let edge = edge_map.get(key).unwrap();
            let direction = key.to_owned().1;
            let mut combined_edge = CombinedEdge { direction, positions: vec![] };
            let mut queue = VecDeque::new();

            let neighbors = edge.get_neighbors();

            mapped_edges.push(*key);

            for neighbor in neighbors {
                queue.push_back(neighbor);
            }

            // Walk the edge
            while let Some(next) = queue.pop_front() {

                if(!edge_map.contains_key(&(next, direction))) {
                    continue;
                }
                if (mapped_edges.contains(&(next, direction))) {
                    continue;
                }

                let next_edge = edge_map.get(&(next, direction)).unwrap();

                combined_edge.positions.push(next);
                mapped_edges.push((next, direction));
                for neighbor in next_edge.get_neighbors() {
                    queue.push_back(neighbor);
                }
            }

            combined_edges.push(combined_edge);
        }

        combined_edges
    }
}

struct SingleEdge {
    position: Point,
    direction: Point,
}

impl SingleEdge {
    fn can_combine_with(&self, other_edge: SingleEdge) -> bool {
        if(self.direction != other_edge.direction){
            return false;
        }

        false
    }
    fn get_neighbors(&self) -> Vec<Point> {
        if(self.direction == SOUTH || self.direction == NORTH) {
            return vec![self.position + EAST, self.position + WEST];
        }
        else if (self.direction == EAST || self.direction == WEST) {
            return vec![self.position + NORTH, self.position + SOUTH];
        }
        Vec::new()
    }
}

struct CombinedEdge {
    positions: Vec<Point>,
    direction: Point
}
#[cfg(test)]
mod tests {
    use crate::days::day_12::{get_regions, Grid};

    #[test]
    fn can_get_regions_for_grid(){
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;

        let grid = Grid::parse(input.to_string());
        let regions = get_regions(&grid);

        assert_eq!(regions.len(), 5);
    }

    #[test]
    fn can_get_regions_areas() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;

        let grid = Grid::parse(input.to_string());
        let regions = get_regions(&grid);

        assert_eq!(regions.iter().nth(0).unwrap().get_area(), 4);
    }

    #[test]
    fn can_get_regions_perimeter() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;

        let grid = Grid::parse(input.to_string());
        let regions = get_regions(&grid);

        let a_region = regions.iter().filter(|r| r.plant == 'A').next().unwrap();
        assert_eq!(a_region.plant, 'A');
        assert_eq!(a_region.calculate_perimeter(), 10);
        assert_eq!(a_region.get_area(), 4);
        assert_eq!(a_region.get_price(), 40);
    }

    #[test]
    fn can_get_region_perimeter_when_it_contains_another_region() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

        let grid = Grid::parse(input.to_string());
        let regions = get_regions(&grid);

        let a_region = regions.iter().filter(|r| r.plant == 'O').next().unwrap();
        assert_eq!(a_region.plant, 'O');
        assert_eq!(a_region.calculate_perimeter(), 36);
        assert_eq!(a_region.get_area(), 21);
    }

    #[test]
    fn can_sum_all_region_prices() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        let grid = Grid::parse(input.to_string());
        let regions = get_regions(&grid);

        let result = regions.iter().map(|x| x.get_price()).sum::<i32>();
        assert_eq!(result, 1930);
    }

    #[test]
    fn can_get_combined_edges() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;

        let grid = Grid::parse(input.to_string());
        let regions = get_regions(&grid);

        let c_region = regions.iter().filter(|r| r.plant == 'A').next().unwrap();
        assert_eq!(c_region.plant, 'A');
        assert_eq!(c_region.get_edges().len(), 4);
    }

    #[test]
    fn can_get_really_weird_combined_edges() {
        let input = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

        let grid = Grid::parse(input.to_string());
        let regions = get_regions(&grid);

        let region = regions.iter().filter(|r| r.plant == 'E').next().unwrap();
        assert_eq!(region.plant, 'E');
        assert_eq!(region.get_edges().len(), 12);
    }

    #[test]
    fn can_get_bulk_discount() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        let grid = Grid::parse(input.to_string());
        let regions = get_regions(&grid);

        for region in &regions {
            println!("{} - {}", region.plant, region.get_price_bulk_discount());
        }
        let result = regions.iter().map(|x| x.get_price_bulk_discount()).sum::<i32>();

        assert_eq!(result, 1206);
    }
}
