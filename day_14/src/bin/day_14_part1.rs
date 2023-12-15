use std::fs;
fn main() {
    let input_path = "input.txt";
    let binding = fs::read_to_string(input_path)
        .unwrap();
    let map = tilt_lever_north(&binding);
    println!("the ans = {}", calc_north_load(&map));
}

fn tilt_lever_north(map: &str) -> Vec<Vec<char>> {
    let mut vec_map: Vec<Vec<char>> = map
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    for i in 0..vec_map.len() {
        for j in 0..vec_map[i].len() {
            if vec_map[i][j] != 'O' {
                continue;
            }
            let mut new_i = i;
            for k in (0..i).rev() {
                if vec_map[k][j] == 'O' ||
                    vec_map[k][j] == '#' {
                        break;
                    }

                if vec_map[k][j] == '.' {
                    new_i = k;
                }
            }
            if new_i != i {
                vec_map[new_i][j] = 'O';
                vec_map[i][j] = '.';
            }
        }
    }
    vec_map.iter()
        .for_each(|line| println!("{:?}", line));
    return vec_map;
}

fn calc_north_load(map: &Vec<Vec<char>>) -> usize {
    let len = map.len();
    let mut load: usize = 0;

    for i in 0..len{
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                load += len - i;
            }
        }
    }

    return load;
}
#[cfg(test)]
mod tests {
    use crate::{tilt_lever_north, calc_north_load};

    #[test]
    fn test_1() {
        let map: String = 
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".to_string();

        let new_map = tilt_lever_north(&map);
        let north_load = calc_north_load(&new_map);
        assert_eq!(136, north_load);
    }

}