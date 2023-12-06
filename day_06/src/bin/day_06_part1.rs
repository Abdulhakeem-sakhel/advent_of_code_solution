use std::fs;
fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("cant't find the file");
    let (time, distance) = get_race(contents.as_str());

    println!("the answer = {}",product_of_error(&time, &distance));
}

fn get_race(records: &str) -> (Vec<u32>, Vec<u32>) {
    
    let time: Vec<u32> = 
    records.lines()
        .nth(0)
        .unwrap()
        .split(" ")
        .filter_map(|st| st.parse::<u32>().ok())
        .collect();
    
    let distance: Vec<u32> =
        records.lines()
        .nth(1)
        .unwrap()
        .split(" ")
        .filter_map(|st| st.parse::<u32>().ok())
        .collect();
    
    println!("{:?}", time);
    println!("{:?}", distance);
    (time, distance)

}
fn product_of_error(time: &Vec<u32>, distance: &Vec<u32>) -> u32 {

    let mut product_error:u32 = 1;
    for i in 0..time.len() {
        let mut num = 0;
        for hold_time in 1..(time[i]-1) {
            let speed = hold_time;
            let calc_distance = speed * (time[i] - hold_time);
            if calc_distance > distance[i] {
                num += 1;
            }
        }
        if num > 0 {
            product_error *= num;
        }
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
    let product_err = product_of_error(&time, &distance);
    assert_eq!(288, product_err);

    }
}