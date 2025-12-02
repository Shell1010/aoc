use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    Left,
    Right
}
impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None
        }
    }
}

pub struct Dial {
    position: i32,
    hit_0: i32
}
impl Dial {
    pub fn new() -> Self {
        Dial { position: 50, hit_0: 0 }
    }

    pub fn move_direction(&mut self, code: &Code) {
        match code.direction {
            Direction::Left => {
                self.position = self.position + code.distance % 100;
                self.hit_0 += ((100 - self.position)%100 + code.distance).div_floor(100).abs()
            },
            Direction::Right => {
                self.position = self.position - code.distance % 100;
                self.hit_0 += ((100 - self.position)%100 - code.distance).div_floor(100).abs()
            }
        }
    }

}

#[derive(Debug)]
pub struct Code {
    direction: Direction,
    distance: i32
}

impl Code {
    pub fn new(direction: Direction, distance: i32) -> Self {
        Code { direction, distance }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let mut chars = s.chars();
        let dir_char = chars.next()?;
        let direction = Direction::from_char(dir_char)?;
        let distance_str: String = chars.collect();
        let distance: i32 = distance_str.parse().ok()?;
        Some(Code::new(direction, distance))
    }

}

pub struct Solution;

impl Solution {
    pub fn parse(input: &str) -> (i32, i32) {
        let directions: Vec<Code> = input
        .trim()
        .split("\n")
        .filter_map(|line| {
            Code::from_str(line)
        }).collect();
        let mut dial = Dial::new();
        let mut part_one = 0; 
        // println!("Parsed {} directions", directions.len());
        for code in directions {
          
            dial.move_direction(&code);
            if dial.position == 0 {
                part_one += 1;
            }
            
        }
        (part_one, dial.hit_0)

    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;
    #[test]
    fn test_code_from_str() {
        let code = Code::from_str("L30").unwrap();
        assert!(matches!(code.direction, Direction::Left));
        assert_eq!(code.distance, 30);
    }

    #[test]
    fn test_dial_movement() {
        let mut dial = Dial::new();
        let code = Code::from_str("L20").unwrap();
        dial.move_direction(&code);
        assert_eq!(dial.position, 70);
    }

    #[test]
    fn test_dial_movement_right() {
        let mut dial = Dial::new();
        let code = Code::from_str("R30").unwrap();
        dial.move_direction(&code);
        assert_eq!(dial.position, 20);
    }

    #[test]
    pub fn test_solution_parse() {
        let mut file = File::open("./inputs/day1.txt").unwrap();
        let mut inputs = String::new();
        file.read_to_string(&mut inputs).unwrap();

        // let inputs = "R1000";
        let (part_one, part_two) = Solution::parse(&inputs);
        println!("{}", part_two)
        
    }
}