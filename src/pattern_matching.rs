// NOTE: The the options enum has 2 variants, one being SOME() and the other being NONE
// This is how we handle null values in rust by using pattern matching on the Option enum

fn find_first_e(s: &String) -> Option<u32> {
    for (idx, char) in s.chars().enumerate() {
        if char == 'e' {
            return Some(idx as u32);
        }
    }

    return None;
}

pub fn pattern_match() {
    let s = String::from("Harshe");

    let result = find_first_e(&s);

    // doing pattern matching below
    match result {
        Some(idx) => {
            println!("index of E is: {}", idx);
        }
        None => {
            println!("E not found!");
        }
    }
}
