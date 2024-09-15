pub fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    }

    return str2;
}

pub fn lifetime() {
    let longest_str;

    let str1 = String::from("small");
    {
        // ERR: Leaving this error here cause it makes it super clear what lifetimes are
        let str2 = String::from("longer");
        longest_str = longest(&str1, &str2);
    }

    println!("Longest string: {}", longest_str);
}
