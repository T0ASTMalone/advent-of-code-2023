use std::collections::HashMap;
use lazy_static::lazy_static;

const RADIX: u32 = 10;

lazy_static! {
    //pub static ref CAT_MAP: HashMap<AnimationActions, AnimationInfo> = vec![
    pub static ref WORDS_MAP: HashMap<&'static str, i32> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]
    .into_iter()
    .collect();
}

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn has_word(sub: &str) -> i32 {
    for &key in WORDS_MAP.keys() {
        if sub.contains(key) {
            return WORDS_MAP.get(key).unwrap().clone();
        }
    }
    -1
}

pub fn parse_line2(line: String) -> usize {
    let mut leftmost: i32 = -1;
    let mut rightmost: i32 = -1;
        
    for (i, char) in line.chars().enumerate() {
        let sub = &line[0..i + 1];
        if char.is_digit(RADIX) {
            leftmost = char.to_digit(RADIX).unwrap() as i32;
            break;
        } else {
            let value = has_word(sub);
            if value != -1 {
                leftmost = value;
                break;
            }
        }
    }
    println!("{}", line);
    for (i, char) in line.chars().rev().enumerate() {
        let sub = &line[i..line.len()];
        if char.is_digit(RADIX) {
            rightmost = char.to_digit(RADIX).unwrap() as i32;
            break;
        } else {
            println!("{}", sub);
            let value = has_word(sub);
            if value != -1 {
                rightmost = value;
                break;
            }
        }
    }
    
    dbg!(leftmost);
    dbg!(rightmost);


    return format!("{}{}", leftmost, rightmost)
        .parse::<usize>()
        .unwrap();
}

pub fn part2(input: &str) -> usize {
    let mut result = 0;

    for val in input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| parse_line2(l.to_owned()))
    {
        result += val;
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    fn parses_a_line() {
        let values: Vec<&str> = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "mxmkjvgsdzfhseightonetwoeight7",
            "eighthree",
        ];
        let result = parse_line2(values[0].to_owned());
        assert_eq!(result, 29);
        let result = parse_line2(values[1].to_owned());
        assert_eq!(result, 83);
        let result = parse_line2(values[2].to_owned());
        assert_eq!(result, 13);
        let result = parse_line2(values[3].to_owned());
        assert_eq!(result, 24);
        let result = parse_line2(values[4].to_owned());
        assert_eq!(result, 42);
        let result = parse_line2(values[5].to_owned());
        assert_eq!(result, 14);
        let result = parse_line2(values[6].to_owned());
        assert_eq!(result, 76);
        let result = parse_line2(values[7].to_owned());
        assert_eq!(result, 87);
        let result = parse_line2(values[8].to_owned());
        assert_eq!(result, 83);
    }

    #[test]
    fn generates_correct_output() {
        let input = include_str!("./test2.txt");
        let output = part2(input);
        assert_eq!(output, 281);
    }
}
