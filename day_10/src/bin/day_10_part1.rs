use std::collections::VecDeque;
use std::fs;
struct Node {
    postilion: (usize, usize),
    cost: u32
}

fn main() {
    println!("Hello, world!");
    let input_file = "input.txt";
    let st = fs::read_to_string(input_file).unwrap();

    let area = get_area(&st);
        let mut starting_position: (usize, usize) = (0, 0); 
            for i in 0..area.len() {
                for j in 0..area[i].len() {
                    if area[i][j] == 'S' {
                        starting_position = (i, j)
                    }
                }
            };
        println!("starting point = {:?}", starting_position);
        let matrix = distance_from_s(&area, starting_position);
        println!("the max = {} ", get_max(&matrix));

}

fn get_max(matrix: &Vec<Vec<Option<u32>>>) ->u32 {
    matrix.iter()
        .map(|row| 
            row.iter()
            .map(|num| num.unwrap_or(0))
            .max().unwrap()
        )
        .max()
        .unwrap()
}

fn get_area(area: &str) -> Vec<Vec<char>> {
    area.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn check_north(current_pos: &(usize, usize), area: &Vec<Vec<char>>, matrix: &mut Vec<Vec<Option<u32>>>) -> bool {
    let (i, j) = *current_pos;
    i.checked_sub(1) != None &&
        (area[i-1][j] == '|' || area[i-1][j] == 'F'|| area[i-1][j] == '7') &&
        matrix[i-1][j] == None
}

fn check_south(current_pos: &(usize, usize), area: &Vec<Vec<char>>, matrix: &mut Vec<Vec<Option<u32>>>) -> bool {
    let size = area.len();
    let (i, j) = *current_pos;
    i < size &&
        (area[i+1][j] == '|' || area[i+1][j] == 'L'|| area[i+1][j] == 'J') &&
        matrix[i+1][j] == None
}

fn check_east(current_pos: &(usize, usize), area: &Vec<Vec<char>>, matrix: &mut Vec<Vec<Option<u32>>>) -> bool {
    let size = area.len();
    let (i, j) = *current_pos;
    j < size &&
        (area[i][j+1] == '-' || area[i][j+1] == '7' || area[i][j+1] == 'J') &&
        matrix[i][j+1] == None
}

fn check_west(current_pos: &(usize, usize), area: &Vec<Vec<char>>, matrix: &mut Vec<Vec<Option<u32>>>) -> bool {
    let (i, j) = *current_pos;
    j.checked_sub(1) != None &&
        (area[i][j-1] == '-' || area[i][j-1] == 'F' || area[i][j-1] == 'L') &&
        matrix[i][j-1] == None
}


fn expand(node: &Node, area: &Vec<Vec<char>>, matrix: &mut Vec<Vec<Option<u32>>>) -> Vec<Node> {
    let mut potential_nodes: Vec<Node> = Vec::new();
    let (i, j) = node.postilion;
    let size = area.len();
    match area[i][j] {
        '|' =>  {
            if check_north(&node.postilion, area, matrix) {
                matrix[i-1][j] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i-1, j), cost: node.cost + 1})
            }
            if check_south(&node.postilion, area, matrix) {
                matrix[i+1][j] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i+1, j), cost: node.cost + 1})
            }
        },
        '-' => {
            if check_east(&node.postilion, area, matrix) {
                matrix[i][j+1] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i, j+1), cost: node.cost + 1});
            }
            if check_west(&node.postilion, area, matrix) {
                matrix[i][j-1] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i, j-1), cost: node.cost + 1});
            }
        }, 
        'L' => {
            if check_north(&node.postilion, area, matrix) {
                matrix[i-1][j] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i-1, j), cost: node.cost + 1})
            }
            if check_east(&node.postilion, area, matrix) {
                matrix[i][j+1] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i, j+1), cost: node.cost + 1});
            }
        },
        'J' => {
            if check_north(&node.postilion, area, matrix) {
                matrix[i-1][j] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i-1, j), cost: node.cost + 1})
            }
            if check_west(&node.postilion, area, matrix) {
                matrix[i][j-1] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i, j-1), cost: node.cost + 1});
            }
        },
        '7' => {
            if check_south(&node.postilion, area, matrix) {
                matrix[i+1][j] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i+1, j), cost: node.cost + 1})
            }

            if check_west(&node.postilion, area, matrix) {
                matrix[i][j-1] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i, j-1), cost: node.cost + 1});
            }
        },
        'F' => {
            if check_south(&node.postilion, area, matrix) {
                matrix[i+1][j] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i+1, j), cost: node.cost + 1})
            }

            if check_east(&node.postilion, area, matrix) {
                matrix[i][j+1] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i, j+1), cost: node.cost + 1});
            }
        },
        'S' => {
            if check_north(&node.postilion, area, matrix) {
                matrix[i-1][j] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i-1, j), cost: node.cost + 1})
            }
            if check_south(&node.postilion, area, matrix) {
                matrix[i+1][j] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i+1, j), cost: node.cost + 1})
            }

            if check_east(&node.postilion, area, matrix) {
                matrix[i][j+1] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i, j+1), cost: node.cost + 1});
            }
            if check_west(&node.postilion, area, matrix) {
                matrix[i][j-1] = Some(node.cost + 1);
                potential_nodes.push(Node {postilion: (i, j-1), cost: node.cost + 1});
            }
        },
        '.' => {
            println!("node pos = {:?}", (i, j));
            println!("what are you doing")
            //panic!("what are you doing")
        },
        _ => panic!("||| UNEXPECTED CHARTER")
    };

    return potential_nodes;
}

fn distance_from_s(area: &Vec<Vec<char>>, starting_position: (usize, usize)) -> Vec<Vec<Option<u32>>> {
    let size = area.len(); // all the example were give of a square
    let mut matrix: Vec<Vec<Option<u32>>> = vec![vec![None; size]; size];
    let mut open: VecDeque<Node> = VecDeque::new();
    open.push_back(Node {postilion: starting_position,cost: 0});
    matrix[starting_position.0][starting_position.1] = Some(0);
    while !open.is_empty() {
        let current_node = open.pop_front().unwrap();
        let e = expand(&current_node, &area, &mut matrix);
        for elm in e {
            open.push_back(elm);
        }
    }

    return  matrix;
}

mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = 
".....
.S-7.
.|.|.
.L-J.
.....";
        let area = get_area(input);
        let mut starting_position: (usize, usize) = (0, 0); 
            for i in 0..area.len() {
                for j in 0..area[i].len() {
                    if area[i][j] == 'S' {
                        starting_position = (i, j)
                    }
                }
            };
        println!("starting point = {:?}", starting_position);
        let matrix = distance_from_s(&area, starting_position);
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == None {
                    print!(".");
                } else {
                    print!("{}", matrix[i][j].unwrap());
                }
            }
            println!("");
        }
    }

    #[test]
    fn test_2() {
        let input = 
"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    let area = get_area(input);
    let mut starting_position: (usize, usize) = (0, 0); 
        for i in 0..area.len() {
            for j in 0..area[i].len() {
                if area[i][j] == 'S' {
                    starting_position = (i, j)
                }
            }
        };
    println!("starting point = {:?}", starting_position);
    let matrix = distance_from_s(&area, starting_position);
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == None {
                print!(".");
            } else {
                print!("{}", matrix[i][j].unwrap());
            }
        }
    }
    let max = get_max(&matrix);
    println!("max = {max}");
    }
}