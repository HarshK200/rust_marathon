pub fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }

    return false;
}

pub fn fibonacci_loop(num: u32) -> u32 {
    if num == 0 || num == 1 {
        return num;
    }

    let mut first = 0;
    let mut second = 1;

    for _ in 0..num - 1 {
        let temp = second;
        second = first + second;
        first = temp;
    }

    return second;
}

pub fn fibonacci_recursive(num: u32) -> u32 {
    //  NOTE: base case
    if num == 0 || num == 1 {
        return num;
    }

    // NOTE: recursive case
    return fibonacci_recursive(num - 1) + fibonacci_recursive(num - 2);
}

pub fn string_length(s: String) -> usize {
    s.chars().count() // NOTE:  implicity return of a value. NO return statement
}
