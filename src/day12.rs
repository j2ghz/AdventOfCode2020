use itertools::Itertools;

#[derive(Clone, Debug)]
struct Ship {
    north: i16,
    east: i16,
    angle: i16,
    wnorth: i16,
    weast: i16,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            north: 0,
            east: 0,
            angle: 0,
            wnorth: 1,
            weast: 10,
        }
    }
}

impl Ship {
    fn move_ship(&self, north: i16, east: i16) -> Ship {
        Ship {
            north: self.north + north,
            east: self.east + east,
            angle: self.angle,
            ..self.clone()
        }
    }

    fn turn_ship(&self, angle: i16) -> Ship {
        Ship {
            north: self.north,
            east: self.east,
            angle: {
                let mut angle = self.angle + angle;
                while angle < 0 {
                    angle += 360;
                }
                while angle >= 360 {
                    angle -= 360;
                }
                angle
            },
            ..self.clone()
        }
    }

    fn forward_ship(&self, distance: i16) -> Ship {
        let mut ship = self.clone();
        match self.angle {
            0 => ship.east += distance,
            90 => ship.north += distance,
            180 => ship.east -= distance,
            270 => ship.north -= distance,
            _ => unreachable!(),
        }
        ship
    }

    fn process(&self, (instr, i): (char, i16)) -> Ship {
        match instr {
            'N' => self.move_ship(i, 0),
            'S' => self.move_ship(-i, 0),
            'E' => self.move_ship(0, i),
            'W' => self.move_ship(0, -i),
            'L' => self.turn_ship(i),
            'R' => self.turn_ship(-i),
            'F' => self.forward_ship(i),
            _ => unreachable!(),
        }
    }

    fn move_waypoint(&self, north: i16, east: i16) -> Ship {
        Ship {
            wnorth: self.wnorth + north,
            weast: self.weast + east,
            ..self.clone()
        }
    }

    fn turn_waypoint(&self, angle: i16) -> Ship {
        let mut ship = self.clone();
        let angle = {
            let mut angle = self.angle + angle;
            while angle < 0 {
                angle += 360;
            }
            while angle >= 360 {
                angle -= 360;
            }
            angle
        };
        // println!("{}", angle);
        match angle {
            0 => (),
            90 => {
                ship.weast = -self.wnorth;
                ship.wnorth = self.weast;
            }
            180 => {
                ship.weast = -self.weast;
                ship.wnorth = -self.wnorth;
            }
            270 => {
                ship.weast = self.wnorth;
                ship.wnorth = -self.weast;
            }
            _ => unreachable!(),
        }
        ship
    }

    fn go_to_waypoint(&self, i: i16) -> Ship {
        Ship {
            north: self.north + (self.wnorth * i),
            east: self.east + (self.weast * i),
            ..self.clone()
        }
    }

    fn process2(&self, (instr, i): (char, i16)) -> Ship {
        match instr {
            'N' => self.move_waypoint(i, 0),
            'S' => self.move_waypoint(-i, 0),
            'E' => self.move_waypoint(0, i),
            'W' => self.move_waypoint(0, -i),
            'L' => self.turn_waypoint(i),
            'R' => self.turn_waypoint(-i),
            'F' => self.go_to_waypoint(i),
            _ => unreachable!(),
        }
    }

    fn manhattan_distance(&self) -> i16 {
        self.north.abs() + self.east.abs()
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<(char, i16)> {
    input
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[1..].parse().unwrap()))
        .collect_vec()
}

#[aoc(day12, part1)]
pub fn part1(input: &[(char, i16)]) -> i16 {
    let result = input.iter().fold(Ship::default(), |s, instr| {
        // println!("{:?}\t{:?}", s, instr);
        s.process(*instr)
    });
    result.manhattan_distance()
}

#[aoc(day12, part2)]
pub fn part2(input: &[(char, i16)]) -> i16 {
    let result = input.iter().fold(Ship::default(), |s, instr| {
        // println!("{:?}\t{:?}", s, instr);
        s.process2(*instr)
    });
    result.manhattan_distance()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    #[test]
    fn part1_example() {
        let input = "\
F10
N3
F7
R90
F11";
        assert_eq!(25, super::part1(&super::input_generator(&input)));
    }
    #[test]
    fn part2_example() {
        let input = "\
F10
N3
F7
R90
F11";
        assert_eq!(286, super::part2(&super::input_generator(&input)));
    }

    #[test]
    fn part1() {
        let input = read_to_string("input/2020/day12.txt").expect("input file missing");
        assert_eq!(362, super::part1(&super::input_generator(&input)));
    }

    #[test]
    fn part2() {
        let input = read_to_string("input/2020/day12.txt").expect("input file missing");
        assert_eq!(29895, super::part2(&super::input_generator(&input)));
    }
}
