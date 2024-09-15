// PERF: This will is VERY! userful since it will be used in anchor (smart contract for rust library)

struct User<'a> {
    name: &'a str,
}

fn structs_with_lifetimes() {
    let user;
    {
        // ERR: Leaving this error here cause it makes it super-clear what lifetimes with structs relation
        let first_name = String::from("harsh");
        user = User { name: &first_name };
    }

    println!("User's name is {}", user.name);
}
