use itertools::Itertools;

pub struct Schedule {
    start: u32,
    busses: Vec<Option<u32>>,
}

impl Schedule {
    fn get_earliest_bus(&self) -> (u32, u32) {
        let (time, bus) = self
            .busses
            .iter()
            .filter_map(|x| x.as_ref())
            .map(|b| {
                let ride = (self.start / b) + 1;
                let time = ride * *b;
                (time - self.start, *b)
            })
            .min_by_key(|(time, _)| *time)
            .unwrap();
        (time, bus)
    }

    fn get_timestamp_subsequent(&self) -> usize {
        (0..)
            .step_by(self.busses.first().unwrap().unwrap() as usize)
            .find(|timestamp| {
                self.busses
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| x.map(|b| (i, b)))
                    .all(|(i, b)| (timestamp + i) as u32 % b == 0)
            })
            .unwrap()
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> anyhow::Result<Schedule> {
    let lines = input.lines().collect_vec();
    Ok(Schedule {
        start: lines[0].parse()?,
        busses: lines[1]
            .split(',')
            .map(|s| s.parse().ok())
            .collect::<Vec<Option<u32>>>(),
    })
}

#[aoc(day13, part1)]
pub fn part1(input: &Schedule) -> u32 {
    let (time, bus) = input.get_earliest_bus();
    time * bus
}

#[aoc(day13, part2)]
pub fn part2(input: &Schedule) -> usize {
    input.get_timestamp_subsequent()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    #[test]
    fn part1() {
        let input = read_to_string("input/2020/day13.txt").expect("input file missing");
        assert_eq!(205, super::part1(&super::input_generator(&input).unwrap()));
    }
    #[test]
    fn part1_example() {
        let input = "\
939
7,13,x,x,59,x,31,19";
        assert_eq!(295, super::part1(&super::input_generator(input).unwrap()));
    }
    #[test]
    fn part2_example() {
        let input = "\
939
7,13,x,x,59,x,31,19";
        assert_eq!(
            1068781,
            super::part2(&super::input_generator(input).unwrap())
        );
    }

    #[test]
    fn part2() {
        let input = read_to_string("input/2020/day13.txt").expect("input file missing");
        assert_eq!(0, super::part2(&super::input_generator(&input).unwrap()));
    }
}
