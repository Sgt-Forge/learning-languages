fn main() {
    println!("===================================================");
    println!("  Chapter 4: Understanding Ownership");
    println!("===================================================");

    let mut s = String::from("Hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print 'Hello, world!'

    println!("===================================================");
    println!("  Chapter 4: Understanding Ownership - Clones");
    println!("===================================================");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    println!("===================================================");
    println!("  Chapter 4: Understanding Ownership - Functions and Ownership");
    println!("===================================================");

    let a = String::from("hello"); // a comes into scope

    takes_ownership(a); // a's value moves into the function, a is no longer valid moving forward

    let x = 5;

    makes_copy(x); // X would move into the function, but i32 is Copy so it's okay to use x again afterwards

    println!("===================================================");
    println!("  Chapter 4: Understanding Ownership - Ownership and Functions");
    println!("===================================================");
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
	let s2 = String::from("hello"); // s2 comes into scope
	let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, whish also moves its return value into s3
    println!("s1 = {}, s3 = {}", s1, s3)
} // x goes out of scope, then s. But s moved so no free happens

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string)
} // Here, some_string goes out of scope and `drop` is called. Memory is freed

fn makes_copy(some_integer: i32) { // here, some integer_comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope, but i32 is on the stack so nothing special happens

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
	let some_string = String::from("yours"); // some_string comes into scope
	some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
	a_string // a_string is returned and moves out to the calling function
}