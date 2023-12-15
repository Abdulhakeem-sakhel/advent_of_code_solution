use std::fs;
fn main() {
    let inputfile = "input.txt";
    let text = fs::read_to_string(inputfile)
        .unwrap();
    println!("hash sum = {}", hash_sum(&text));
}

fn hash(text: &str) -> u32 {
    let mut hash_value = 0;

    for ch in text.as_bytes() {
        //println!("{ch}");
        hash_value += *ch as u32;
        hash_value *= 17;
        hash_value %= 256;
    }

    return hash_value;
}

fn hash_sum(input: &str) -> usize {
    input.split(",")
        .filter(|ch| !ch.is_empty())
        .map(|text| hash(text) as usize)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_hash() {
        let text1 = "rn=1";
        let text2 = "cm-";
        //println!("{}",hash("HASH"));
        assert_eq!(30, hash(text1));
        assert_eq!(253, hash(text2));
    }

    #[test]
    fn test_1() {
        let input = 
"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(1320, hash_sum(input));

    }
}