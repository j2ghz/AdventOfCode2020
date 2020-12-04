use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

fn main() {
    println!("Hello, world!");
}

fn is_valid(passport: &str) -> bool {
    let re = Regex::new(r"([a-z]+):\S+\s*").unwrap();
    let mut props = re
        .captures_iter(passport)
        .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
        .collect::<HashMap<&str, &str>>();

    println!("{:?}", props);
    false
}

fn valid_byr(value: &str) -> bool {
    let num: u16 = value.parse().unwrap();
    num >= 1920 && num <= 2002
}

fn valid_iyr(value: &str) -> bool {
    let num: u16 = value.parse().unwrap();
    num >= 2010 && num <= 2020
}

fn valid_eyr(value: &str) -> bool {
    let num: u16 = value.parse().unwrap();
    num >= 2020 && num <= 2030
}

fn valid_hgt(value: &str) -> bool {
    let matches = Regex::new(r"(\d+)(cm|in)").unwrap().captures(value);
    if let Some(matches) = matches {
        let num: usize = matches.get(1).unwrap().as_str().parse().unwrap();
        match matches.get(2).map(|m| m.as_str()) {
            Some("in") => num >= 59 && num <= 76,
            Some("cm") => num >= 150 && num <= 193,
            _ => false,
        }
    } else {
        false
    }
}

fn matches_in_file(filename: &str) -> usize {
    let contents = read_to_string(filename).unwrap();
    contents
        .split("\r\n\r\n")
        .filter(|pass| is_valid(*pass))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{is_valid, matches_in_file, valid_byr, valid_hgt};

    #[test]
    fn field_validation() {
        assert_eq!(true, valid_byr("2002"));
        assert_eq!(false, valid_byr("2003"));

        assert_eq!(true, valid_hgt("60in"));
        assert_eq!(true, valid_hgt("190cm"));
        assert_eq!(false, valid_hgt("190in"));
        assert_eq!(false, valid_hgt("190"));
    }

    macro_rules! passport_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (expected,input) = $value;
                assert_eq!(expected, is_valid(input));
            }
        )*
        }
    }

    passport_tests! {
        t1: (true, "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"),
        t2: (false, "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929"),
        t3: (true, "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm"),
        t4: (false, "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in"),
    }

    #[test]
    fn example() {
        assert_eq!(2, matches_in_file("example.txt"));
    }
    #[test]
    fn main() {
        assert_eq!(226, matches_in_file("input.txt"));
    }
}
