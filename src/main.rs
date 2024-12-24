fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, test!");
}

#[test]
fn test_variable() {
    let name = "Yonathan Setiadi";
    println!("Name is {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Yonathan Setiadi";
    println!("Name is {}", name);

    name = "Arif";
    println!("Name is {}", name);

}

#[test]
fn test_number() {
    let  a = 100000000;
    println!("Name is {}", a);

    let b:i128 = 1000000000000000000000000000000;
    println!("Name is {}", b);

}

#[test]

fn test_augmented() {
    let mut a = 10;
    println!("a {}", a);

    a += 10;
    println!("a {}", a);

}

#[test]
fn array() {
    let mut array = ['a', 'b', 'c'];

    println!("{:?}", array);

    let idx0 = array[0];
    println!("{}", idx0);

    array[1] = 'd';
    println!("{:?}", array);

    let lenght = array.len();
    println!("{}", lenght);
}


#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Yonathan");
    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Asi");
    println!("{}, {}", a, b);
}

#[test]
fn string_type() {
    let mut name = String::from("Yonathan");
    println!("{}", name);

    name.push_str(" Setiadi"); // bukan mutable jadi gak bisa dipakai methodnya
    println!("{}", name);

    let new_name = name.replace("Yonathan", "Jonathan");
    println!("{}", new_name);
}

#[test]
fn ownership_movement() {
    let own = String::from("my owner");

    let owl = own.clone(); // method clone pada tipe data heap membuat copy data
    let bowl = own; // tanpa method clone memindahkan ownership ke variable lain

    println!("{} {}", owl, bowl);
}

#[test]
fn if_expression() {
    let value = 7;

    let result = if value >= 8 {
        "Good"
    } else if value >= 6{
        "OK"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter;
        } else if counter%2 == 0 {
            continue;
        }
    };

    println!("{}", result);
}

#[test]

fn loop_label() {
    let mut number = 0;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10{
                break 'outer;
            }
    
            println!("{} * {} = {}", number, i, number * i);

            i+= 1;
            if i > 5 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter);
        }

        counter +=1;
    }
}

#[test]
fn array_iteration() {
    let array:[&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value: {}", value);
    }
}

#[test]
fn range() {
    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    let array:[&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range  {
        println!("Value: {}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let range = 0..=4;
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    let array:[&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range  {
        println!("Value: {}", array[i]);
    }
}

fn factorial_loop(n:i32) -> i32 {
    if n < 1 {
        return 0;
    }
    
    if n == 1 {
        return 1;
    }

    n * factorial_loop(n - 1)
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(-5);
    println!("{}",result);
}

fn print_number(number: i32) {
    println!("{}", number);
}

fn hi(str: String) {
    println!("Hi {}", str);
}

#[test]
fn test_print_number() {
    let num = 24;

    print_number(num);
    println!("my number {}", num);
}

#[test]
fn test_hi() {
    let name = String::from("Nathan");

    hi(name.clone());
    println!("My name {}", name);
}

fn fullname(first: String, last: String) -> (String, String, String) {
    let full_name = format!("{} {}", first, last);

    (first, last, full_name)
}

#[test]
fn test_fullname() {
    let first = String::from("Yonathan");
    let last = String::from("Setiadi");
    let (first, last, name) = fullname(first, last);

    println!("{}", name);
    println!("{}", first);
    println!("{}", last);
}