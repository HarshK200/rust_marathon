use core::fmt;

pub struct User {
    pub name: String,
    pub age: u32,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

