pub fn string_slice() {
    let fullname = String::from("harsh kapadia");

    // NOTE: string slice
    let firstname = find_first_word(&fullname);

    println!("first name: {}", firstname);
}

// returns a string slice
fn find_first_word(fullname: &String) -> &str {
    let mut first_name_len = 0;

    for char in fullname.chars() {
        if char == ' ' {
            break;
        }
        first_name_len += 1;
    }

    return &fullname[0..first_name_len];
}
