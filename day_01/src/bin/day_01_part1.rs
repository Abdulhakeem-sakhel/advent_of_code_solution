use std::fs;
fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("cant't find the file");
    println!("the answer = {}",sum_of_document(contents.as_str()));
}

fn extract_two_digit(word: &str) -> usize {
    let first_digit = word.chars().nth(word.find(|c: char| c.is_numeric()).unwrap()).unwrap().to_digit(10).unwrap() as usize; 
    let second_digit = word.chars().nth(word.rfind(|c: char| c.is_numeric()).unwrap()).unwrap().to_digit(10).unwrap() as usize; 
    first_digit * 10+
    second_digit
}

fn sum_of_document(document: &str) -> usize {
    document
    .lines()
    .map(|x| extract_two_digit(x.trim()))
    .reduce(|a, b| a + b)
    .unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
        let document = 
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        
        let result = sum_of_document(document);
        let expected_result:usize = 142;

        assert_eq!(result, expected_result);

    }
}