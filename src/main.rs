mod first;
mod second;
mod model;
mod third;

use first::hello;
use second::hello as hello_second;
use model::User;

#[test]
fn test_module() {
    let user = User::create_person(
        String::from("Yonathan"), 
        String::from("Setiadi"),
        String::from("nathan"), 
        String::from("nathan@mail.com"), 
        32);

    user.say_hello("Burhan");
}




#[test]
fn test_use() {
    hello();
    hello_second();
    first::second::third::hello();
}

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

fn fullname(first: &String, last: &String) -> String {
    format!("{} {}", first, last)
}

#[test]
fn test_fullname() {
    let first = String::from("Yonathan");
    let last = String::from("Setiadi");
    let fullname = fullname(&first, &last);

    println!("{}", fullname);
}

fn change_value(value: &mut String) {
    value.push_str("Test");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Makanan");
    change_value(&mut value);
    println!("{}", value);
}

fn get_fullname(first: &String, last: &String) -> String {
    format!("{} {}", first, last)
}

#[test]
fn test_get_fullname() {
    let first = String::from("Yonathan");
    let last = String::from("Setiadi");
    let fullname = get_fullname(&first, &last);

    println!("{}", fullname);
}

#[test]
fn slice_references() {
    let array: [i32;10]= [0,1,2,3,4,5,6,7,8,9];
    let full: &[i32] = &array[..];
    println!("{:?}", full);

    let part_1: &[i32] = &array[..5];
    println!("{:?}", part_1);

    let part_2: &[i32] = &array[6..];
    println!("{:?}", part_2);

}

#[test]
fn string_slice() {
    let name = String::from("Yonathan Setiadi");
    let first_name: &str = &name[..8];
    println!("{}", first_name);
}

struct Person {
    first_name: String,
    last_name: String,
    age: u8
}

#[test]
fn test_struct_person() {
    let first_name = String::from("Yonathan");
    let person = Person{
        age: 3,
        first_name,
        last_name : String::from("Setiadi"),
    };

    let person2 = Person{
        first_name: person.first_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };

    print_person(&person2);
    print_person(&person);
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

struct GeoPoint(f64, f64);

#[test]
fn test_geopoint() {
    let geopoint = GeoPoint(-3.00, 4.22);

    println!("{}", geopoint.0);
    println!("{}", geopoint.1);
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {} ",name, self.first_name);
    }
}

#[test]
fn test_method() {
    let person = Person{
        age: 3,
        first_name: String::from("Yonathan"),
        last_name : String::from("Setiadi"),
    };

    person.say_hello("Arif");
    println!("{}", person.first_name);
}

impl GeoPoint {
    fn new(long: f64, lat: f64) ->GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_assosiated_function() {
    let geo_point = GeoPoint::new(0.78, -8.99);
    println!("{}", geo_point.0);
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with bank {} and number {}, amount {}", bank, number, amount);
            }
            Payment::EWallet(wallet,number ) => {
                println!("Paying with wallet {} and number {}, amount {}", wallet, number, amount);
            }
        }
    }
}

#[test]
fn test_payment() {
    let payment = Payment::BankTransfer(String::from("BCA"), String::from("8326297364324"));
    payment.pay(3000);
    let payment1 = Payment::EWallet(String::from("Gopay"), String::from("8326297364324"));
    payment1.pay(67898);
    let payment2 = Payment::CreditCard(String::from("8326297364324"));
    payment2.pay(67898);
}

#[test]
fn test_enum() {
    let level = Level::Premium;

    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

#[test]
fn test_match_value() {
    let name = "Amin";

    match name {
        "Eko" | "Budi"=> {
            println!("Hello Eko!");
        }
        "Joko" => {
            println!("Hello Budi");
        }
        other => {
            println!("Bangun kau {}!", other);
        }
    }
}

#[test]
fn test_range_pattern() {
    let value = 75;

    match  value {
        75..=100 => {
            println!("Great!");
        }
        60..75 => {
            println!("Good!");
        }
        40..=59 => {
            println!("Bad!");
        }
        0..40 => {
            println!("Too Bad");
        }
        other => {
            println!("Not a valid number {}", other);
        }
    }

    let person = Person{
        first_name: String::from("Budi"),
        last_name: String::from("Karya"),
        age: 8,
    };

    match  person {
        Person{first_name, age, ..} => {
            println!("Name {} age {} ", first_name, age);
        }
    }

}

// Type Alias
type Age = u8;
type IdentityNumber = String;

struct Costumer {
    name: String,
    age: Age,
    id_number: IdentityNumber,
}
#[test]
fn test_customer() {
    let customer = Costumer{
        name: String::from("Yonathan"),
        age: 32,
        id_number: String::from("1234567890"),
    };

    println!("Name: {}", customer.name);
    println!("Age: {}", customer.age);
    println!("ID Number: {}", customer.id_number);
}

trait CanSayHello {
    fn hello(&self) -> String{
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodbye {
    fn say_goodbye(&self) -> String;
    fn say_goodbye_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.first_name)
    }
}

impl CanSayGoodbye for Person {
    fn say_goodbye(&self) -> String {
        format!("Goodbye, my name is {}", self.first_name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.first_name)
    }
}

fn can_say_hello(person: &impl CanSayHello) {
    println!("{}", person.say_hello());
}

fn say_hello_goodbye(value: &(impl CanSayHello + CanSayGoodbye)) {
    println!("{}", value.say_hello());
    println!("{}", value.say_goodbye());
}

#[test]
fn test_trait() {
    let person = Person{
        first_name: String::from("Yonathan"),
        last_name: String::from("Setiadi"),
        age: 32,
    };

    can_say_hello(&person);

    println!("{}", person.say_hello_to("Arif"));
    println!("{}", person.hello());
    println!("{}", person.say_goodbye());
    println!("{}", person.say_goodbye_to("Arif"));

    say_hello_goodbye(&person);

    CanSayHello::say_hello(&person);
    Person::say_hello(&person, "Arif");
}

struct SimplePerson {
    name: String
}

impl CanSayHello for SimplePerson {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.name)
    }
}

fn create_simple_person(name: String) -> impl CanSayHello {
    SimplePerson{ name }
}

#[test]
fn test_return_trait() {
    let person = create_simple_person(String::from("Yonathan"));
    println!("{}", person.say_hello());
    println!("{}", person.say_hello_to("Arif"));
}

trait CanSay: CanSayHello + CanSayGoodbye {
    fn say(&self);
}

struct SimpleMan {
    name: String
}

impl CanSay for SimpleMan {
    fn say(&self) {
        println!("{}", self.say_hello());
        println!("{}", self.say_goodbye());
    }
}

impl CanSayHello for SimpleMan {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.name)
    }
}

impl CanSayGoodbye for SimpleMan {
    fn say_goodbye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}

#[test] 
fn test_trait_object() {
    let person = SimpleMan{
        name: String::from("Yonathan"),
    };

    person.say();
    println!("{}", person.say_hello());
    println!("{}", person.say_goodbye());
}

struct Point<T = i32> {
    x: T,
    y: T,
}

#[test]
fn test_generic_struct() {
    let point = Point::<i64>{
        x: 10,
        y: 20,
    };

    println!("x: {}, y: {}", point.x, point.y);

    let point2 = Point::<f64>{
        x: 10.5,
        y: 20.5,
    };

    println!("x: {}, y: {}", point2.x, point2.y);
}

enum Value<T> {
    NONE,
    Value(T),
}

#[test]
fn test_generic_enum() {
    let value = Value::<i64>::Value(10);

    match value {
        Value::Value(v) => {
            println!("Value: {}", v);
        }
        Value::NONE => {
            println!("No Value");
        }
    }
}

struct Hi<T: CanSayHello> {
    value: T,
}

#[test]
fn test_generic_bound() {

    let hi = Hi::<SimplePerson>{
        value: SimplePerson { 
            name: String::from("Yonathan")
        }
    };

    println!("{}", hi.value.say_hello());
}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

#[test]
fn test_generic_function() {
    let a = 10;
    let b = 20;

    let min_value = min(a, b);
    println!("Min: {}", min_value);

    let c = 10.5;
    let d = 20.5;

    let min_value2 = min(c, d);
    println!("Min: {}", min_value2);
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_method() {
    let point = Point::new(10, 20);
    println!("x: {}, y: {}", point.get_x(), point.get_y());

    let point2 = Point::new(10.5, 20.5);
    println!("x: {}, y: {}", point2.get_x(), point2.get_y());
    println!("{}", point2.get_value());
}

trait GetValue<T> where T: PartialOrd {
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T> where T: PartialOrd {
    fn get_value(&self) -> &T {
        &self.x
    }
}

// Overloadable Operator
use core::ops::Add;
use core::ops::AddAssign;
use std::fmt::Debug;
use std::fmt::Display;

struct Apple {
    quantity: u32
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity
        }
    }
}

impl AddAssign for Apple {
    fn add_assign(&mut self, rhs: Self) {
        self.quantity += rhs.quantity;
    }
}


#[test]
fn test_add() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 10 };

    let apple3 = apple1 + apple2;
    println!("Apple 3: {}", apple3.quantity);

    let mut apple4 = Apple { quantity: 0 };
    apple4 += apple3;
    println!("Apple 4: {}", apple4.quantity);
}

// End Overloadable Operator

// Option Enum


fn double(value: Option<i32>) -> Option<i32> {
    match value {
        Some(v) => Some(v * 2),
        None => None,
    }
}

#[test]
fn test_option() {
    let value = Some(10);
    let result = double(value);
    println!("{:?}", result);

    let value2 = None;
    let result2 = double(value2);
    println!("{:?}", result2);

    let value3 = Some(20);
    let result3 = double(value3);
    println!("{:?}", result3);
}
// End Option Enum

// Comparisson

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_comparisson() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 20 };

    println!("Apple 1 == Apple 2: {}", apple1 == apple2);
    println!("Apple 1 != Apple 2: {}", apple1 != apple2);
    println!("Apple 1 < Apple 2: {}", apple1 < apple2);
    println!("Apple 1 > Apple 2: {}", apple1 > apple2);
}
// End Comparisson

// String Manipulation
#[test]
fn test_string_manipulation() {
    let name = String::from("Yonathan");
    println!("{}", name);
    println!("{}", name.to_ascii_lowercase());
    println!("{}", name.to_ascii_uppercase());
    println!("{}", name.to_lowercase());
    println!("{}", name.to_uppercase());
    // name.push_str(" Setiadi");
    // println!("{}", name);

    // let new_name = name.replace("Yonathan", "Jonathan");
    // println!("{}", new_name);

    // let split_name: Vec<&str> = new_name.split_whitespace().collect();
    // println!("{:?}", split_name);
}
// End String Manipulation

// Formatting
struct Category {
    id: String,
    name: String,   
}

impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Category: {} - {}", self.id, self.name)
    }
}

#[test]
fn test_formatting() {
    let category = Category {
        id: String::from("1"),
        name: String::from("Food"),
    };

    println!("{:?}", category);
    println!("Category: {:#?}", category);
    println!("{}", category);
}#[test]

// End Formatting

// Closure
#[test]
fn test_closure() {
    let sum = |a: i32, b: i32| a + b;
    let sub = |a: i32, b: i32| a - b;

    let result = sum(10, 20);
    let result2 = sub(20, 10);
    println!("Result: {}", result);
    println!("Result2: {}", result2);
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("{}", result);
}

#[test]
fn test_closure_parameter() {
    print_with_filter(String::from("Yonathan"), |a:String| a.to_uppercase());
}

#[test]
fn test_closure_scope() {
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Counter: {}", counter);
    };
    increment();
    increment();
    increment();

    println!("Final Counter: {}", counter);
}

struct Counter {
    count: i32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn get_count(&self) -> i32 {
        self.count
    }
}

#[test]
fn test_closure_scope_using_struct() {
    let mut counter = Counter::new();

    counter.increment();
    counter.increment();
    counter.increment();

    println!("Final Counter: {}", counter.get_count());
}
// End Closure

// Sequence
// Vector = struktur datanya mirip stack (tumpukan)

#[test]
fn test_vector() {
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Yonathan"));
    names.push(String::from("Setiadi"));
    names.push(String::from("Arif"));

    for name in &names {
        println!("{}", name);
    }

    println!("{:?}", names);
}

// VecDeque = struktur datanya mirip dengan array pada umumnya bisa diakses dari depan dan belakang
use std::collections::VecDeque;
#[test]
fn test_vector_deque() {
    let mut names: VecDeque<String> = VecDeque::new();
    names.push_back(String::from("Setiadi"));
    names.push_front(String::from("Yonathan"));

    for name in &names {
        println!("{}", name);
    }

    println!("{}", names[1]);
}

use std::collections::LinkedList;
#[test]
fn test_linked_list() {
    let mut names: LinkedList<String> = LinkedList::new();
    names.push_back(String::from("Setiadi"));
    names.push_front(String::from("Yonathan"));
    names.push_back(String::from("Arif"));
    names.push_front(String::from("Budi"));

    for name in &names {
        println!("{}", name);
    }
}
// End Sequence

// Map

// HashMap
use std::collections::HashMap;
#[test]
fn test_hashmap() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("name"), String::from("Yonathan"));
    map.insert(String::from("age"), String::from("32"));

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    println!("{}", map.get("name").unwrap());
    println!("{}", map.get_mut("age").unwrap());
}

use std::collections::BTreeMap;
#[test]
fn test_btreemap() {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert(String::from("name"), String::from("Yonathan"));
    map.insert(String::from("age"), String::from("32"));
    map.insert(String::from("address"), String::from("Jakarta"));

    for entry in &map {
        println!("{}: {}", entry.0, entry.1);
    }
}

use std::collections::HashSet;

#[test]
fn test_hash_set() {
    let mut names: HashSet<String>= HashSet::new();
    names.insert(String::from("Yonathan"));
    names.insert(String::from("Setiadi"));
    names.insert(String::from("Yonathan"));
    names.insert(String::from("Setiadi"));

    for name in &names {
        println!("{}", name);
    }
}

use std::collections::BTreeSet;
#[test]
fn test_btree_set() {
    let mut names: BTreeSet<String>= BTreeSet::new();
    names.insert(String::from("Yonathan"));
    names.insert(String::from("Setiadi"));
    names.insert(String::from("Yonathan"));
    names.insert(String::from("Setiadi"));

    for name in &names {
        println!("{}", name);
    }
}

// Iterator
#[test]
fn test_iterator() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut iter = array.iter();

    println!("Count total iter: {}", iter.clone().count());

    while let Some(value) = iter.next() {
        println!("{}", value);
    }
}

#[test]
fn test_iterator_method() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", vector);
    
    let sum: i32 = vector.iter().sum();
    println!("Sum: {}", sum);

    let count: usize = vector.iter().count();
    println!("Count: {}", count);

    let doubled: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let filtered: Vec<&i32> = vector.iter().filter(|x| *x % 2 == 0).collect();
    println!("Filtered: {:?}", filtered);
}

// Error Handling

fn connect_database(host: Option<String>){
    match (host) {
        Some(host) => {
            println!("Connect to database {}", host);
        }
        None => {
            println!("No host");
        }
    }
}

#[test]
fn test_connect_db() {
    let host = Some(String::from(""));
    let host2 = None;
    connect_database(host2);
    connect_database(host);
}

fn connect_cache(  host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => {
            Ok(host)
        }
        None => {
            Err("no cache provider".to_string())
        }
    }
}

#[test]
fn test_connect_cache() {
    let cache = connect_cache(None);
    match cache {
        Ok(host) => {
            println!("Cache connected to {}", host);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

fn connect_email(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => {
            Ok(host)
        }
        None => {
            Err("no email provider".to_string())
        }
    }
}

fn connect_application(host: Option<String>) -> Result<String, String> {
    connect_cache(host.clone())?;
    connect_email(host.clone())?;

    Ok(format!("Connect to application"))
}

#[test]
fn test_connect_application() {
    let result = connect_application(None);
    match result {
        Ok(host) => {
            println!("Success with message {}", host);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

#[test]
fn test_dangling_reference() {
    let r: &i32;
    {
        let x = 5;
        // r = &x;
    }

    r = &40;

    println!("{}", r)
}

fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

#[test]

fn test_lifetime_annotation() {
    let first = "Arifin";
    let second = "Ahmad";

    let result = longest(first, second);
    println!("{}", result)
}

struct Student<'a> {
    name: &'a str
}

impl<'a> Student<'a> {
    fn longest_name(&self, student: &Student<'a>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

fn longest_student_name<'a>(student1: &Student<'a>, student2 :&Student<'a>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
    }
}

#[test]
fn test_longest_student() {
    let student1 = Student{
        name: "Bumi"
    };
    let student2 = Student{
        name: "Nugraha"
    };

    let result = longest_student_name(&student1, &student2);
    println!("{}", result);

    let resultMethod = student1.longest_name(&student2);
    println!("{}", resultMethod);
}

struct Teacher <'a, ID> where ID : Ord {
    id: ID,
    name: &'a str,
}

#[test]
fn test_lifetime_generic() {
    let teacher = Teacher{
        id: 30,
        name: "Arifin",
    };

    println!("{}", teacher.id);
    println!("{}", teacher.name);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    website: String,
}

#[test]
fn test_attribute_drive() {
    let company = Company{
        name: "Sicpe".to_string(),
        location: "Jakarta".to_string(),
        website: "https://jakarta.com/sicep".to_string(),
    };

    println!("{:?}", company);
}

#[test]
fn test_box() {
    let value: Box<i32> = Box::new(10);
    println!("{}", value);

    display_number(*value);
    display_number_reference(&value);
}

fn display_number(value: i32) {
    println!("{}", value);
}

fn display_number_reference(value: &i32) {
    println!("{}", value);
}

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End
}

#[test]
fn test_box_enum() {
    let category_dell = Box::new(ProductCategory::Of("Dell".to_string(), Box::new(ProductCategory::End)));
    let category = ProductCategory::Of(
        "laptop".to_string(),
        category_dell,
    );

    println!("{:?}", category);
}

use std::ops::Deref;

struct MyValue<T> {
    value: T,
}

impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn call_hello_name(name: &String) {
    println!("Hello {}", name);
}

#[test]
fn test_dereference_trait() {
    let value = MyValue{ value: 20};
    let real_value = *value;

    println!("{}", real_value);

    let name = MyValue{ value: "Ahmad".to_string()};

    call_hello_name(&name);
}

struct Book {
    title: String,
}

// untuk melakukan proses apapun sebelum datanya dihapus dari memory
impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping book {}", self.title);
    }
}

#[test]
fn test_drop_book() {
    let book = Book {
        title: "Jumanji".to_string(),
    };

    println!("Book title {}", book.title);
}

use std::rc::Rc;
enum Brand {
    Of(String, Rc<Brand>),
    End
}


#[test]
fn test_multiple_ownership() {
    // let apple = ProductCategory::Of("Apple".to_string(), Box::new(ProductCategory::End));
    // let phone = ProductCategory::Of("Smartphone".to_string(), Box::new(apple));
    // let laptop = ProductCategory::Of("Laptop".to_string(), Box::new(apple));

    let apple = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    println!("Apple reference count: {} ",  Rc::strong_count(&apple));

    let laptop = Brand::Of("Laptop".to_string(), Rc::clone(&apple));
    println!("Apple reference count: {} ",  Rc::strong_count(&apple));

    {
        let smartphone = Brand::Of("Smartphone".to_string(), Rc::clone(&apple));
        println!("Apple reference count: {} ",  Rc::strong_count(&apple));
    }

    println!("Apple reference count: {} ",  Rc::strong_count(&apple));

}

use core::cell::RefCell;
#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>,
}

#[test]
fn test_reff_cell() {
    let seller = Seller{name: RefCell::new("Arif".to_string()), active: RefCell::new(true)};

    {
        let mut result = seller.name.borrow_mut();
        *result = "Budi".to_string();
    }
    let activation = seller.active.borrow();


    println!("{:?} {:?}", seller.name, activation);
}

macro_rules! hi {
    () => {
        println!("Hi macro!")        
    };
    ($name: expr) => {
        println!("Hi, {}!", $name)
    }
}

#[test]
fn test_macro() {
    hi!();
    hi!("Bulma");
    hi!{
        "gila"
    };
}

macro_rules! iterate {
    ($array: expr) => {
        for i in $array {
            println!("{}", i);
        }
    };
    ($($item: expr), *) => {
        $(
            println!("{}", $item);

        )*
    }
}

#[test]
fn test_iterate_macro() {
    iterate!([1,2,3,4,5,6,7]);
    iterate!(1,2,3,4,5,6);
}
