pub fn read_file() {
    // NOTE: the path is relative to the project src where Cargo.toml file is
    let file_data = std::fs::read_to_string("test.txt");

    match file_data {
        Result::Ok(s) => {
            println!("{}", s);
        }
        Result::Err(e) => {
            println!( "Error occured: what did you do dumbass?, here's the log:\n{}", e)
        }
    }
}
