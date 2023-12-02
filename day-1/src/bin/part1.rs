fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

pub fn parse_line(line: &str) -> usize {
    // get all digits
    const RADIX: u32 = 10;
    let only_digits: Vec<char> = line.chars().filter(|c| c.is_digit(RADIX)).collect();
    // get first and last digits
    let first = only_digits[0];
    let last = only_digits[only_digits.len() - 1];
    // concat as strings
    format!("{}{}", first, last).parse::<usize>().unwrap()
}


pub fn part1(input: &str) -> usize {
    let mut result = 0;
    for val in input.split('\n').filter(|l| l.len() > 0).map(|l| parse_line(l)) {
        result += val;
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_a_line() {
        let values: Vec<&str> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let result = parse_line(&values[0]);
        assert_eq!(result, 12);
        let result = parse_line(&values[1]);
        assert_eq!(result, 38);
        let result = parse_line(&values[2]);
        assert_eq!(result, 15);
        let result = parse_line(&values[3]);
        assert_eq!(result, 77);
    }

    #[test]
    fn generates_correct_output() {
        let input = include_str!("./test.txt");
        let output = part1(input);
        assert_eq!(output, 142);
    }
}


