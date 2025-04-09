pub struct User {
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    age: u8,
}

impl User{
    pub fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {} ",name, self.first_name);
    }

    pub fn create_person(first_name: String, last_name: String, username: String, email: String, age: u8) -> User {
        User{
            first_name,
            last_name,
            username,
            email,
            age
        }
    }
}