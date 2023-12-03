use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("cant't find the file");
    println!("the answer = {}",sum_engine_schematic(contents.as_str()));
}

fn get_number(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    //get the left side first
    if  i >= matrix.len() ||
        j >= matrix[i].len() ||
        !matrix[i][j].is_digit(10) 
          {
            return  0;
        }
    
    let mut number: usize = 0;
    let mut pointer_left: usize = j;
    while pointer_left.checked_sub(1) != None {
        if matrix[i][pointer_left-1].is_numeric() {
            pointer_left -= 1;
        } else {
            break;
        }
    }

    while pointer_left < matrix[i].len() && matrix[i][pointer_left].is_numeric()  {
        number *= 10;
        number += matrix[i][pointer_left].to_digit(10).unwrap() as usize;
        //matrix[i][pointer_left] = '.';
        pointer_left += 1;
    }
    return number;
}

fn sum_engine_schematic(engine_schematic: &str) -> usize {
    let mut sum: usize = 0;
    let mut matrix: Vec<Vec<char>> = 
    engine_schematic
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch)
                .collect()
        })
        .collect();
    for (i, line) in matrix.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if ch.is_ascii_alphabetic() || ch.is_digit(10) || *ch == '.' {
                continue;
            }
            //checking the left
            if j.checked_sub(1) != None {
                sum += get_number(&matrix, i, j-1);
            }
            //checking the right
            sum += get_number(&matrix, i, j+1);
            //checking the top
            if i.checked_sub(1) != None {
                let result = get_number(&matrix, i-1, j);
                if result > 0 {
                    sum += result;
                }
                else {
                    //checking the left-top
                    if i.checked_sub(1) != None && j.checked_sub(1) != None {
                        sum += get_number(&matrix, i-1, j-1);
                    }
                    //checking the right-top
                    if i.checked_sub(1) != None {
                    sum += get_number(&matrix, i-1, j+1);
                }
            }
            //checking the bottom
            let result = get_number(&matrix, i+1, j);
            if result > 0 {
                sum += result;
            } else {
                //checking the left-bottom
                if j.checked_sub(1) != None {
                    sum += get_number(&matrix, i+1, j-1);
                }
                //checking the right-bottom
                sum += get_number(&matrix, i+1, j+1);
                }
            }
        }
    };

    return sum;
   // return sum
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let engine_schematic = 
"467..114.
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(4361,sum_engine_schematic(engine_schematic));
    }
}
