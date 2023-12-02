use std::ops::Add;
use std::fs;
fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("cant't find the file");
   println!("the answer = {}",sum_power_set_cubes_games(contents.as_str()));

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

fn fewest_number_cubes(full_game: &str) -> GameSet {
    let mut number_cubes: GameSet = GameSet { red_cube_number: 0, green_cube_number: 0, blue_cube_number: 0};

    full_game.split(":").nth(1).unwrap()
    .split(";")
    .for_each(|cube:&str| {
        cube.split(',')
            .for_each(|set:&str| {
                let number: usize = set.trim_start().split(" ").nth(0).unwrap().parse().unwrap();
                let cube_color: &str = set.trim_start().split(" ").nth(1).unwrap();
        
                match cube_color {
                    "red" => if number > number_cubes.red_cube_number {
                        number_cubes.red_cube_number = number;
                    },
                    "green" => if number > number_cubes.green_cube_number {
                        number_cubes.green_cube_number = number;
                    },
                    "blue" => if number > number_cubes.blue_cube_number {
                        number_cubes.blue_cube_number = number;
                    },
                    default => panic!("i did,'t received a correct color the word i received is {default}")
                };
            });
    });

    number_cubes
}

fn power_set_cubes(cube_set: GameSet) -> usize {
    cube_set.red_cube_number * 
        cube_set.green_cube_number *
        cube_set.blue_cube_number
}

fn sum_power_set_cubes_games(full_games: &str) -> usize {
    full_games.lines()
        .map(|game| power_set_cubes(fewest_number_cubes(game)))
        .reduce(|a, b| a + b)
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

        let test_round = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let test_set = GameSet{red_cube_number:4, green_cube_number: 2, blue_cube_number: 6};
        assert_eq!(test_set, fewest_number_cubes(test_round));
        assert_eq!(48, power_set_cubes( fewest_number_cubes(test_round)));
        assert_eq!(2286, sum_power_set_cubes_games(games))
    }
}