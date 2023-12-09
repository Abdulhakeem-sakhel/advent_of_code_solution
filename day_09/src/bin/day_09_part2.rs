use std::fs;
fn main() {
    let file_path = "input.txt";
    let binding = fs::read_to_string(file_path)
        .unwrap();
    let sensor_reading = binding.as_str();
    println!("the answer = {}",sum_of_extrapolated_values(sensor_reading));
}

fn only_have_zeros(seq: &Vec<i64>) -> bool {
    for num in seq {
        if *num != 0 {
            return false;
        }
    };

    return true;
}

fn find_prev_value_in_sequence (sequence : &str) -> i64 {
    let seq: Vec<i64> =
    sequence.split(" ")
        .filter_map(|num| num.parse::<i64>().ok())
        .collect();
    let mut history: Vec<Vec<i64>> = Vec::new();
    history.push(seq);
    let mut depth:usize = 0;
    loop {

        if only_have_zeros(&history[depth]) {
            break;
        }

        let mut next_seq: Vec<i64> = Vec::new();
        for i in  0..(history[depth].len() - 1) {
            next_seq.push((history[depth][i + 1] - history[depth][i]).try_into().unwrap());
        }
        history.push(next_seq);
        depth += 1;
    }

    //println!("{:?}", history);

    history[depth].push(0);
    depth -= 1;

    loop { // or while Ok(depth-=1)
        let prev_pattern =  history[depth].first().unwrap() - history[depth + 1].first().unwrap();
        history[depth].insert(0,prev_pattern);


        if depth.checked_sub(1) == None {
            break;
        };
        depth -= 1;
    }
    //println!("{:?}", history);

    return history[0].first().unwrap().clone();
}

fn sum_of_extrapolated_values(sensor_reading: &str) -> i64 {
    return  
    sensor_reading.lines()
        .filter(|line| !line.is_empty())
        .map(|line| find_prev_value_in_sequence(line))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let sensor_reading = 
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        find_prev_value_in_sequence("1 3 6 10 15 21");
        assert_eq!(2, sum_of_extrapolated_values(sensor_reading));
    } 
}