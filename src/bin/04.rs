use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(4);

fn check_passport(input: &str, check_function: fn(&HashMap<&str, &str>) -> bool) -> Option<u32> {
    let mut passport = HashMap::new();
    let mut count = 0;

    for line in input.lines() {
        if line.is_empty() {
            if check_function(&passport) {
                count += 1;
            }
            passport.clear();
        }
        for word in line.split_whitespace() {
            let mut parts = word.split(':');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            passport.insert(key, value);
        }
    }
    if check_function(&passport) {
        count += 1;
    }

    Some(count)
}

pub fn part_one(input: &str) -> Option<u32> {
    check_passport(input, |passport| {
        passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid"))
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    check_passport(input, |passport| {
        (match passport.get("byr") {
            Some(byr) => match byr.parse::<u32>() {
                Ok(byr) => (1920..=2002).contains(&byr),
                Err(_) => {
                    return false;
                }
            },
            None => {
                return false;
            }
        }) && (match passport.get("iyr") {
            Some(iyr) => match iyr.parse::<u32>() {
                Ok(iyr) => (2010..=2020).contains(&iyr),
                Err(_) => {
                    return false;
                }
            },
            None => {
                return false;
            }
        }) && (match passport.get("eyr") {
            Some(eyr) => match eyr.parse::<u32>() {
                Ok(eyr) => (2020..=2030).contains(&eyr),
                Err(_) => {
                    return false;
                }
            },
            None => {
                return false;
            }
        }) && (match passport.get("hgt") {
            Some(hgt) => {
                let (height_str, unit) = hgt.split_at(hgt.len() - 2);
                let height = height_str.parse::<u32>().unwrap();
                match unit {
                    "cm" => (150..=193).contains(&height),
                    "in" => (59..=76).contains(&height),
                    _ => false,
                }
            }
            None => {
                return false;
            }
        }) && (match passport.get("hcl") {
            Some(hcl) => Regex::new(r"#[0-9a-f]{6}").unwrap().is_match(hcl),
            None => {
                return false;
            }
        }) && (match passport.get("ecl") {
            Some(ecl) => Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$")
                .unwrap()
                .is_match(ecl),
            None => {
                return false;
            }
        }) && (match passport.get("pid") {
            Some(pid) => Regex::new(r"^[0-9]{9}$").unwrap().is_match(pid),
            None => {
                return false;
            }
        }) && (passport.len() == 7 && !passport.contains_key("cid") || passport.len() == 8)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
