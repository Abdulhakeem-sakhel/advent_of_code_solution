use std::ops::Add;
use std::fs;
fn main() {
    let file_path = "input.txt";
    let bag_content = GameSet {
        red_cube_number: 12,
        green_cube_number: 13,
        blue_cube_number: 14
    };
    let contents = fs::read_to_string(file_path)
        .expect("cant't find the file");
    println!("the answer = {}",sum_of_id_possible_games(&bag_content,contents.as_str()));

}

#[derive(PartialEq, Eq, Debug)]
struct GameSet {
    red_cube_number: usize,
    green_cube_number: usize,
    blue_cube_number: usize,
}

impl Add for GameSet {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        GameSet {
            red_cube_number: self.red_cube_number + rhs.red_cube_number,
            blue_cube_number: self.blue_cube_number + rhs.blue_cube_number,
            green_cube_number: self.green_cube_number + rhs.green_cube_number,
        }
    }
}


fn get_cube_numbers(round: &str) -> GameSet {
    round.split(",")
    .map(|cube| ->  GameSet{
        let number: usize = cube.trim_start().split(" ").nth(0).unwrap().parse().unwrap();
        let cube_color: &str = cube.trim_start().split(" ").nth(1).unwrap();
        
        match cube_color {
            "red" => GameSet { red_cube_number: number, green_cube_number: 0, blue_cube_number: 0 },
            "green" => GameSet { red_cube_number: 0, green_cube_number: number, blue_cube_number: 0 },
            "blue" => GameSet { red_cube_number: 0, green_cube_number: 0, blue_cube_number: number },
            default => panic!("i did,'t received a correct color the word i received is {default}")
        }

    })
    .reduce(|a, b| a + b).unwrap()
}

fn possible_game(bag_content: &GameSet, full_game: &str) -> bool {
    full_game.split(":").nth(1).unwrap().trim_start()
        .split(";")
        .map(|round| get_cube_numbers(round))
        .map(|cube_set| 
                cube_set.red_cube_number <= bag_content.red_cube_number &&
                cube_set.blue_cube_number <= bag_content.blue_cube_number &&
                cube_set.green_cube_number <= bag_content.green_cube_number
            )
        .reduce(|a, b| a && b)
        .unwrap()
}
fn sum_of_id_possible_games(bag_content: &GameSet, full_games: &str) -> usize {
    full_games.lines()
    .map(|full_game: &str| -> usize {
        if possible_game(&bag_content, full_game) {
            full_game.split(":").nth(0).unwrap()
                .split(" ").nth(1).unwrap().parse().unwrap()
        } else {
            0
        }
    })
    .reduce(|a, b|  a + b)
    .unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
        let games = 
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let bag_content = GameSet {
            red_cube_number: 12,
            green_cube_number: 13,
            blue_cube_number: 14
        };

        let test_round = "5 blue, 4 red, 13 green";
        let test_test = GameSet{red_cube_number:4, green_cube_number: 13, blue_cube_number: 5};
        assert_eq!(get_cube_numbers(test_round), test_test);
        assert_eq!(true, possible_game(&bag_content, "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"));
        assert_eq!(8, sum_of_id_possible_games(&bag_content, games))

    }
}