use crate::third::hello as hello_third;

pub fn hello() {
    println!("Hello from first module");

    hello_third();
}

pub mod second {
    pub mod third {
        pub fn hello() {
            super::super::hello();
            // crate::first::hello();
        }
    }
}