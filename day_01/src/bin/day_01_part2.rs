use std::fs;
fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("cant't find the file");
    println!("the answer = {}",sum_of_document(contents.as_str()));
}

//if it find a spelled digit it return it in usize
fn is_digit_spelled(s: &String) -> Option<usize> {
    let st = s.as_str();    
        if st.contains("one") {
            Some(1)
        } else if st.contains("two"){ 
            Some(2)
        } else if st.contains("three"){ 
            Some(3)
        } else if st.contains("four"){ 
            Some(4)
        } else if st.contains("five"){ 
            Some(5)
        } else if st.contains("six"){ 
            Some(6)
        } else if st.contains("seven"){ 
            Some(7)
        } else if st.contains("eight"){ 
            Some(8)
        } else if st.contains("nine"){ 
            Some(9)
        } 
        else {
            None
        }
        
        
}

fn extract_two_digit(word: &str) -> usize {
    
    let mut spelled_litter: String = String::new();
    let mut first_digit: usize = 0;
    for ch in word.chars() {
        if ch.is_digit(10) {
            first_digit = ch.to_digit(10).unwrap() as usize;
            break;
        }
        spelled_litter.push(ch);
        match is_digit_spelled(&spelled_litter) {
            Some(t) => {
                first_digit = t;
                break;
            }
            None => continue,
        }
    };
    let mut spelled_litter: String = String::new();
    let mut second_digit: usize = 0;
    for ch in word.chars().rev() {
        if ch.is_digit(10) {
            second_digit = ch.to_digit(10).unwrap() as usize;
            break;
        }
        spelled_litter.insert(0, ch);
        match is_digit_spelled(&spelled_litter) {
            Some(t) => {
                second_digit = t;
                break;
            }
            None => continue,
        }
    };
    first_digit * 10 + second_digit
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
    #[test]
    fn it_works2() {
        let document = 
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        
        let result = sum_of_document(document);
        let expected_result:usize = 281;

        assert_eq!(result, expected_result);

    }
}