fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

const RADIX: u32 = 10;
pub fn parse_line(line: &str) -> usize {
    // walk from left 
    //  when you encounter the first digit
    //      set leftmost
    // walk from right
    //  when you encounter the first digit 
    //      set rightmost
    let mut leftmost: i32 = -1;
    let mut rightmost: i32 = -1;


    for char in line.chars() {
        if char.is_digit(RADIX) {
            leftmost = char.to_digit(RADIX).unwrap() as i32;
            break;
        }
    }
    
    for char in line.chars().rev() {
        if char.is_digit(RADIX) {
            rightmost = char.to_digit(RADIX).unwrap() as i32;
            break;
        }
    }

    return format!("{}{}", leftmost, rightmost).parse::<usize>().unwrap();
    
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

