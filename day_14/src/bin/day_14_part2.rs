use std::{fs, collections::HashMap};
fn main() {
    let input_path = "input.txt";
    let binding = fs::read_to_string(input_path)
        .unwrap();
    let mut vec_map = get_vec_map(&binding);
    let mut hash_map = HashMap::new();
    for _ in 1..10000 {
        cycles_tilt(&mut vec_map, 1);
        let load = calc_north_load(& vec_map);
        if hash_map.get(&load) == None {
            hash_map.insert(load, 1);
        } else {
            hash_map.insert(load, hash_map.get(&load).unwrap() + 1);
        }
    }
    let keys: Vec<usize> = hash_map.keys().map(|ch| *ch).collect();
    println!("{:?}", keys);
}

fn get_vec_map(map: &str) -> Vec<Vec<char>> {
    let vec_map: Vec<Vec<char>> = map
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    return vec_map;
}

fn tilt_lever_north(vec_map:&mut Vec<Vec<char>>) {
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
}

fn tilt_lever_south(vec_map:&mut Vec<Vec<char>>) {
    for i in (0..vec_map.len()).rev() {
        for j in 0..vec_map[i].len() {
            if vec_map[i][j] != 'O' {
                continue;
            }
            let mut new_i = i;
            for k in i+1..vec_map.len() {
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
}

fn tilt_lever_east(vec_map:&mut Vec<Vec<char>>) {
    for i in 0..vec_map.len() {
        for j in (0..vec_map[i].len()).rev() {
            if vec_map[i][j] != 'O' {
                continue;
            }
            let mut new_j = j;
            for k in j+1..vec_map[i].len() {
                if vec_map[i][k] == 'O' ||
                    vec_map[i][k] == '#' {
                        break;
                    }

                if vec_map[i][k] == '.' {
                    new_j = k;
                }
            }
            if new_j != j {
                vec_map[i][new_j] = 'O';
                vec_map[i][j] = '.';
            }
        }
    }
}

fn tilt_lever_west(vec_map: &mut Vec<Vec<char>>) {
    for i in 0..vec_map.len() {
        for j in 0..vec_map[i].len() {
            if vec_map[i][j] != 'O' {
                continue;
            }
            let mut new_j = j;
            for k in (0..j).rev() {
                if vec_map[i][k] == 'O' ||
                    vec_map[i][k] == '#' {
                        break;
                    }

                if vec_map[i][k] == '.' {
                    new_j = k;
                }
            }
            if new_j != j {
                vec_map[i][new_j] = 'O';
                vec_map[i][j] = '.';
            }
        }
    }
}

fn cycles_tilt(vec_map: &mut Vec<Vec<char>>, number_of_cycle: usize) {
    for _ in 0..number_of_cycle {
        tilt_lever_north(vec_map);
        tilt_lever_west(vec_map);
        tilt_lever_south(vec_map);
        tilt_lever_east(vec_map);
    }
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
    use super::*;

    #[test]
    fn test_1() {
        let map: String = 
"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".to_string();

        let mut vec_map = get_vec_map(&map);
        //tilt_lever_east(&mut vec_map);
        cycles_tilt(&mut vec_map, 100000);
        vec_map.iter()
            .for_each(|line| {
                line.iter()
                    .for_each(|ch| print!("{ch}"));
                println!("");
            });
        let mut hash_map = HashMap::new();
        let mut number_of_matches: Vec<u32> = Vec::new();
        for i in 1..=1000 {
            cycles_tilt(&mut vec_map, 1);
            if calc_north_load(&vec_map) == 64 {
                number_of_matches.push(i);
            }
            let load = calc_north_load(&vec_map);
            if hash_map.get(&load) == None {
                hash_map.insert(load, 1);
            } else {
                hash_map.insert(load, hash_map.get(&load).unwrap() + 1);
            }
        }
        println!("{:?}", hash_map);
        //println!("{:?}", number_of_matches);
    }

}