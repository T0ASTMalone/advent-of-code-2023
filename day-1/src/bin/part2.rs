use std::{
    collections::{HashMap, HashSet},
    usize::{MAX, MIN},
};

const WORDS: [&'static str; 14] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "twone",
    "oneight",
    "sevenine",
    "eightwo",
    "eighthree",
];

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_number_str(number: &str) -> &str {
    match number {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",

        "twone" => "21",
        "oneight" => "18",
        "sevenine" => "79",
        "eightwo" => "82",
        "eighthree" => "83",
        _ => "",
    }
}
pub fn concat_first_and_last(t: String) -> usize {
    const RADIX: u32 = 10;
    let only_digits: Vec<char> = t.chars().filter(|c| c.is_digit(RADIX)).collect();
    // get first and last digits
    let first = only_digits[0];
    let last = only_digits[only_digits.len() - 1];
    /*
    dbg!(first);
    dbg!(last);
    */
    // concat as strings
    format!("{}{}", first, last).parse::<usize>().unwrap()
}

pub fn parse_line2(line: String) -> usize {
    let save = line.clone();

    if line.len() < 1 {
        return 0;
    }

    let len = line.len();
    let f = line.chars().nth(0).unwrap();
    let l = line.chars().nth(len - 1).unwrap();

    if f.is_digit(10) && l.is_digit(10) {
        return concat_first_and_last(line);
    }

    // replace all words with valid values
    let mut map: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();

    let mut map2: HashMap<usize, &str> = HashMap::new();

    for w in WORDS {
        if line.contains(w) {
            dbg!(w);

            let indices = save.match_indices(w);

            map.insert(w, indices.clone().collect());

            for (i, u) in indices {
                map2.insert(i, u);
            }
        }
    }

    let kes = map2.keys();

    let min_key = kes.clone().min();
    let max_key = kes.clone().max();

    let min_w = match min_key {
       Some(m) => map2.get(m),
       _ => None,
    };

    let max_w = match max_key {
        Some(m) => map2.get(m) ,
        _ => None
    };

    let replaced_min: String = match min_w {
        Some(w) => line.replacen(w, get_number_str(w), 1),
        _ => line,
    };

    let mut t = replaced_min.clone();

    if let Some(m) = max_w {
        if t.contains(m) {
            t = replaced_min.replace(m, get_number_str(m));
        }
    }

    return concat_first_and_last(t);
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

    #[test]
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
