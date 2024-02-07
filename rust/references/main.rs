fn main() {
    println!("===================================================");
    println!("  Chapter 4.2: References");
    println!("===================================================");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    println!("===================================================");
    println!("  Chapter 4.2: Mutable References");
    println!("===================================================");
    let mut s = String::from("hello");

	change(&mut s);
	println!("{}", s);

    println!("===================================================");
    println!("  Chapter 4.2: Multiple References to One Datum");
    println!("===================================================");

    // { // scope 1 - errors
    //     let mut s = String::from("hello");
        
    //     // errors
    //     let r1 = &mut s;
    //     let r2 = &mut s;
        
    //     println!("{}, {}", r1, r2);
    // }
    
    // { // scope 2 - errors
    //     let mut s = String::from("hello");
        
    //     // errors
    //     let r1 = &mut s;
    //     let r2 = &s;
        
    //     println!("{}, {}", r1, r2);
    // }
    
    // { // scope 3 - errors
    //     let mut s = String::from("hello");
        
    //     // errors
    //     let r2 = &s;
    //     let r1 = &mut s;
    
    //     println!("{}, {}", r1, r2);
    // }
    
    { // scope 4 - All good!
        let mut s = String::from("hello");
        
        let r1 = &s;
        let r2 = &s;
    
        println!("s = {}, r1 = {}, r2 = {}", s, r1, r2);
    }

    println!("===================================================");
    println!("  Chapter 4.2: Reference Scope");
    println!("===================================================");
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let r3 = &mut s;

    println!("this is okay: {}", r3);
    {
        println!("===================================================");
        println!("  Chapter 4.2: Slices");
        println!("===================================================");

        // let mut s = String::from("hello world");
        // let slice = first_word(&s);
        // s.clear();
        // println!("slice = {}", slice); // This will cause an error because clear() needs a mutable reference but the immutable refernce scope overlaps

        let s = String::from("hello world!");
        let slice = first_word(&s);
        println!("slice = {}", slice); // slice will go out of scope here because this is the last time it's used
        s.clear(); // This is fine because slice is now out of scope.
        println!("s = {}", s);
    }
}

fn calculate_length(s: &String) -> usize {
	s.len()
}

fn change(some_string: &mut String) {
	some_string.push_str(", world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..]
}