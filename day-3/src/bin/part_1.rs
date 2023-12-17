fn main() {
    println!("Hello main!")
}

pub fn parse_input(input: &str) -> Vec<&str> {
    return input.split("\n").collect();
}
const EMPTY: char = ' ';

pub fn get_adjacent_indices(x: i32, y: i32, length: i32 ) -> Vec<(i32, i32)> {
    let y_vec = vec![y - 1, y, y + 1];
    let mut x_vec = vec![x + 1, x];

    for i in 1..length + 1 {
        x_vec.push(x - i);
    }

    let mut indices: Vec<(i32, i32)> = vec![];

    for y_2 in y_vec {
       for x_2 in x_vec.clone() {

            if (x_2 <= (x - length) || x_2 > x) || (y_2 > y || y_2 < y) {
                indices.push((x_2, y_2));
            }

        }
    }

    return indices;
}

pub fn find_adjacent_numbers(lines: Vec<&str>) -> Vec<i32> {
    // create adjecten nubmers array
    let mut numbers: Vec<i32> = vec![];
    let mut number: String = "".to_owned();

    // walk array
    for (y, line) in lines.iter().enumerate() {
        // walk line
        for (x, char) in line.chars().enumerate() {
            let mut found = EMPTY;

            // check if number 
            if char.is_digit(10) {
                number.push(char.to_owned());
                continue;
            }

            if char != '.' && !char.is_digit(10) && !number.is_empty() {
                println!("Found symbol right after number");
                numbers.push(number.as_str().parse::<i32>().unwrap());
                number = "".to_owned();
                continue;
            }

            // when . found
            if char == '.' && !number.is_empty() {
                println!("Found dot and number is present");
                // check surrounding indices 
                let indices = get_adjacent_indices((x - 1) as i32, y as i32, number.len() as i32)
                    .into_iter()
                    .filter(|&(x, y)| x >= 0 && x < (line.len() - 1) as i32 && y >= 0 as i32 && y < (lines.len() - 1) as i32);

                for (x, y) in indices {
                    println!("x: {}, y: {}", x, y);
                    // get char at indices
                    let line = lines.get(y as usize);
                    
                    if let Some(&l) = line {

                        let char = l.chars().nth(x as usize);

                        if let Some(c) = char {

                            if !c.is_digit(10) && c != '.' {
                                // found symbol
                                found = c;
                                break;
                            }
                        }
                    }
                }
            }

            if found != EMPTY {
                println!("Found symbol");
                // adjecent number 
                numbers.push(number.as_str().parse::<i32>().unwrap());
                number = "".to_owned();
                found = EMPTY;
            } else if found == EMPTY && number.parse::<usize>().is_ok() {
                number = "".to_owned();
                found = EMPTY;
            }

        }
    }

    return numbers;
}

fn sum_schematic(input: &str) -> i32 {
    let lines = parse_input(input);
    let numbers = find_adjacent_numbers(lines);
    return numbers.iter().sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn splits_line() {
        let input = include_str!("./test_1.txt");
        let lines = parse_input(input);
        assert_eq!(lines[0], "467..114..");
    } 

    #[test]
    fn gets_adjacent_indices() {
        let x = 3;
        let y = 2;
        let len = 2;

        let indices = get_adjacent_indices(x, y, len);
        assert_eq!(indices, vec![(4, 1), (3, 1), (2, 1), (1, 1), (4, 2), (1, 2), (4, 3), (3, 3), (2, 3), (1, 3)]);
    }

    #[test]
    fn finds_numbers_agacent_to_symbols() {
        let input = include_str!("./test_1.txt");
        let sum = sum_schematic(input);
        assert_eq!(sum, 4361);
    }
}
