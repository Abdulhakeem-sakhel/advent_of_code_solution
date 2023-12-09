use std::{fs, usize};
fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("cant't find the file");
    let (time, distance) = get_race(contents.as_str());

   println!("the answer = {}",product_of_error(time, distance));
}

fn get_race(records: &str) -> (usize, usize) {
    
   let time = 
    records.lines()
        .nth(0)
        .unwrap()
        .split(" ")
        .filter_map(|st| st.parse::<u32>().ok())
        .map(|num| num.to_string())
        .reduce(|num1, num2| format!("{num1}{num2}"))
        .unwrap()
        .parse::<usize>()
        .unwrap();

    
    let distance =
        records.lines()
        .nth(1)
        .unwrap()
        .split(" ")
        .filter_map(|st| st.parse::<u32>().ok())
        .map(|num| num.to_string())
        .reduce(|num1, num2| format!("{num1}{num2}"))
        .unwrap()
        .parse::<usize>()
        .unwrap();
    
    println!("{:?}", time);
    println!("{:?}", distance);
    (time, distance)

}
fn product_of_error(time: usize , distance: usize) -> usize {

    let mut product_error:usize = 1;
        let mut num = 0;
    for hold_time in 1..(time-1) {
            let speed = hold_time;
            let calc_distance = speed * (time - hold_time);
            if calc_distance > distance {
                num += 1;
            }
        }
        if num > 0 {
            product_error *= num;
        }

    return  product_error;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn boat_test() {
        let records = 
"Time:      7  15   30
Distance:  9  40  200
";

    let (time, distance) = get_race(records);
    let product_err = product_of_error(time, distance);
    assert_eq!(71503, product_err);

    }
}