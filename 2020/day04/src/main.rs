use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

fn main() {
    println!("Hello, world!");
}

fn get_passports(file: &str) -> std::vec::Vec<String> {
    read_to_string(file)
        .unwrap()
        .split("\r\n\r\n")
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

fn is_valid(passport: &str) -> bool {
    let re = Regex::new(r"([a-z]+):(\S+)\s*").unwrap();
    let props = re
        .captures_iter(passport)
        .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
        .collect::<HashMap<&str, &str>>();

    println!("{:?}", props);
    props.get("byr").map(|v| valid_byr(v)).unwrap_or(false)
        && props.get("iyr").map(|v| valid_iyr(v)).unwrap_or(false)
        && props.get("eyr").map(|v| valid_eyr(v)).unwrap_or(false)
        && props.get("hgt").map(|v| valid_hgt(v)).unwrap_or(false)
        && props.get("hcl").map(|v| valid_hcl(v)).unwrap_or(false)
        && props.get("ecl").map(|v| valid_ecl(v)).unwrap_or(false)
        && props.get("pid").map(|v| valid_pid(v)).unwrap_or(false)
}

fn valid_byr(value: &str) -> bool {
    let num: u16 = value.parse().unwrap();
    (1920..=2002).contains(&num)
}

fn valid_iyr(value: &str) -> bool {
    let num: u16 = value.parse().unwrap();
    (2010..=2020).contains(&num)
}

fn valid_eyr(value: &str) -> bool {
    let num: u16 = value.parse().unwrap();
    (2020..=2030).contains(&num)
}

fn valid_hgt(value: &str) -> bool {
    let matches = Regex::new(r"(\d+)(cm|in)").unwrap().captures(value);
    if let Some(matches) = matches {
        let num: usize = matches.get(1).unwrap().as_str().parse().unwrap();
        match matches.get(2).map(|m| m.as_str()) {
            Some("in") => (59..=76).contains(&num),
            Some("cm") => (150..=193).contains(&num),
            _ => false,
        }
    } else {
        false
    }
}

fn valid_hcl(value: &str) -> bool {
    Regex::new(r"#[0-9a-f]{6}").unwrap().is_match(value)
}

fn valid_ecl(value: &str) -> bool {
    Regex::new(r"amb|blu|brn|gry|grn|hzl|oth")
        .unwrap()
        .is_match(value)
}

fn valid_pid(value: &str) -> bool {
    Regex::new(r"^\d{9}$").unwrap().is_match(value)
}

pub fn matches_in_file(filename: &str) -> usize {
    let passports = get_passports(filename);
    passports.iter().filter(|p| is_valid(p)).count()
}

#[cfg(test)]
mod tests {
    use crate::{is_valid, matches_in_file, valid_byr, valid_ecl, valid_hcl, valid_hgt, valid_pid};

    #[test]
    fn field_validation() {
        assert_eq!(true, valid_byr("2002"));
        assert_eq!(false, valid_byr("2003"));

        assert_eq!(true, valid_hgt("60in"));
        assert_eq!(true, valid_hgt("190cm"));
        assert_eq!(false, valid_hgt("190in"));
        assert_eq!(false, valid_hgt("190"));

        assert_eq!(true, valid_hcl("#123abc"));
        assert_eq!(false, valid_hcl("#123abz"));
        assert_eq!(false, valid_hcl("123abc"));

        assert_eq!(true, valid_ecl("brn"));
        assert_eq!(false, valid_ecl("wat"));

        assert_eq!(true, valid_pid("000000001"));
        assert_eq!(false, valid_pid("0123456789"));
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
    //#[test]
    // fn part1() {
    //     assert_eq!(226, matches_in_file("input.txt"));
    // }
    #[test]
    fn part2() {
        assert_eq!(160, matches_in_file("input.txt"));
    }
}
