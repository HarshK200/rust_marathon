pub fn print_current_time() {
    // local time
    let current_time = chrono::Local::now();

    println!("current time: {}", current_time.format("%d.%h.%Y %I:%M %p"));
}
