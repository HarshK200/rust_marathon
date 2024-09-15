pub trait Summarize {
    // NOTE: This is the default implementation for when this trait get's implemented and no custom
    // implementation is provided
    fn summarize(&self) -> String {
        return String::from("this is a summery");
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summarize for User {
    fn summarize(&self) -> String {
        return format!(
            "user's name is {} and user is {} years old",
            self.name, self.age,
        );
    }
}

// NOTE: the generic T should implement the Summarize trait
fn notify<T: Summarize>(param: &T) {
    println!("{}", param.summarize());
}

pub fn traits() {
    let user = User {
        name: String::from("harsh"),
        age: 20,
    };

    notify(&user);

    // ERR: the below code wont't work because the "t" struct doesn't implement's the summarize trait

    // struct Test {}
    // let t = Test {};
    // notify(t);
}
