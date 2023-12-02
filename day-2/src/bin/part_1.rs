struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}

impl CubeSet {
    fn new(red: usize, blue: usize, green: usize) -> Self {
        CubeSet { red, blue, green }
    }
}

struct Game {
    game: usize,
    sets: Vec<CubeSet>,
}

impl Game {
    fn new(game: usize, sets: Vec<CubeSet>) -> Self {
        Game { game, sets }
    }
}

// parse games
fn parse_games(input: &str) -> Vec<&str> {
    let mut split_str: Vec<&str> = input.split('\n').collect();
    // removing last empty string
    split_str.pop();
    return split_str;
}

// parse sets and game name
fn parse_cube_sets(input: &str) -> Game {
    let mut cube_sets: Vec<CubeSet> = vec![];
    let game_and_sets: Vec<&str> = input.split(':').collect();

    let game: Vec<&str> = game_and_sets.get(0).unwrap().split(' ').collect();
    let game_number: usize = game.get(1).unwrap().parse::<usize>().unwrap();
    let sets: Vec<&str> = game_and_sets.get(1).unwrap().split(';').collect();

    for s in sets {
        let split_by_comma: Vec<Vec<&str>> = s
            .split(',')
            .map(|c| c.trim().split(' ').collect())
            .collect();

        let mut set: CubeSet = CubeSet::new(0, 0, 0);
        // loop over each and create struct
        for val in split_by_comma {
            match val.get(1) {
                Some(c) => match c.to_owned() {
                    "red" => {
                        set.red = val.get(0).unwrap().to_owned().parse::<usize>().unwrap();
                    }
                    "blue" => {
                        set.blue = val.get(0).unwrap().to_owned().parse::<usize>().unwrap();
                    }
                    "green" => {
                        set.green = val.get(0).unwrap().to_owned().parse::<usize>().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        cube_sets.push(set);
    }
    return Game::new(game_number, cube_sets);
}

fn is_valid_game(game: &Game, set: &CubeSet) -> bool {
    for s in &game.sets {
        if set.red < s.red || set.blue < s.blue || set.green < s.green {
            return false;
        }
    }
    return true;
}

fn part_1(input: &str, set: CubeSet) -> usize {
    let games: usize = parse_games(input)
        .into_iter()
        .map(|g| parse_cube_sets(g))
        .filter(|g| is_valid_game(&g, &set))
        .map(|g| g.game)
        .sum();

    return games;
}

fn main() {
    let input = include_str!("./input_1.txt");
    let set = CubeSet::new(12, 14, 13);
    let game_sum = part_1(input, set);
    //⭐ 2237
    println!("Game Sum: {}", game_sum);
}

#[cfg(test)]
mod test {
    use super::*;

    const GAME_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    const GAME_2: &str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";

    #[test]
    fn parses_games() {
        let file = include_str!("./test_1.txt");
        let parsed_file = parse_games(file);
        // matches game 1
        assert_eq!(parsed_file.get(0).unwrap().to_owned(), GAME_1.to_owned());
        // matches game 2
        assert_eq!(parsed_file.get(1).unwrap().to_owned(), GAME_2.to_owned());
        // removes last empty value
        assert_ne!(
            "".to_owned(),
            parsed_file.get(parsed_file.len() - 1).unwrap().to_owned()
        );
    }

    #[test]
    fn parses_cube_sets() {
        let game = parse_cube_sets(GAME_1);
        let cube_sets = game.sets;
        assert_eq!(3, cube_sets.get(0).unwrap().blue);
        assert_eq!(4, cube_sets.get(0).unwrap().red);
        assert_eq!(1, cube_sets.get(1).unwrap().red);
        assert_eq!(2, cube_sets.get(1).unwrap().green);
        assert_eq!(6, cube_sets.get(1).unwrap().blue);
        assert_eq!(2, cube_sets.get(2).unwrap().green);

        assert_eq!(1, game.game);
    }

    #[test]
    fn validates_game() {
        let cube_set = CubeSet::new(5, 6, 4);
        let cube_set_2 = CubeSet::new(1, 1, 1);
        let game = parse_cube_sets(GAME_1);

        assert_eq!(true, is_valid_game(&game, &cube_set));
        assert_eq!(false, is_valid_game(&game, &cube_set_2));
    }

    #[test]
    fn test_1() {
        let file = include_str!("./test_1.txt");
        let set = CubeSet::new(12, 14, 13);
        let game_sum = part_1(file, set);
        assert_eq!(8, game_sum);
    }
}
