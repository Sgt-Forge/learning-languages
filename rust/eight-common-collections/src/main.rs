use std::Collections::HashMap;

fn print_type_of<T>(_value: &T) {
    println!("Type: {}", std::any::type_name::<T>());
}

fn main() {
    println!("==================================================");
    println!("Chapter 8: Common Collections");
    println!("==================================================");
    {
        let v: Vec<i32> = Vec::new(); // annotation because we haven't added anything to the vector so rust doesn't know what type the vector is
        println!("v = {:?}", v); // will print out square brackets with the contents of the vector (empty right now)
    }

    {
        let mut v = Vec::new(); // Don't need annotation here because we push i32 on later and the compiler can infer. Need mut thought
        v.push(1);
        v.push(2);
        v.push(3);
        println!("v = {:?}", v);
    }

    {
        let v = vec![4, 5, 6]; // No annotation here
        println!("v = {:?}", v);
    }

    {
        let v = vec![4, 5, 6]; // No annotation here
        let a = v[0];
        let b = &v[1];
        println!("a = {}", a);
        print_type_of(&a);
        println!("b = {}", b);
        print_type_of(&b);
    }
    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2]; // This is from the book. As far as I can tell, there is no reason why this is a reference
        println!("The third element is {third}");
    
        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }    
    }
    {
        let mut v = vec![1, 2, 3, 4, 5];

        let third: i32 = v[2]; 
        println!("The third element is {third}");
    
        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }    

        for i in v.iter() {
            println!("i = {}", i);
        }

        v.clear();
    }

    println!("==================================================");
    println!("Chapter 8: More on String");
    println!("==================================================");
    {
        let s1 = "initial contents".to_string();
        let s2 = String::from("initial contents");
        let s3 = String::new();
        let hello = String::from("Здравствуйте");
        println!("s1 = {s1}, s2 = {s2}, s3 = {s3}, hello = {hello}");
    }

    println!("==================================================");
    println!("Chapter 8: Updating and growing string");
    println!("==================================================");
    {
        let mut s1 = String::from("foo");
        s1.push_str("bar");
        println!("s1 = {s1}");
    }
    {
        let mut s = String::from("foo");
        let s2 = "bar";
        s.push_str(s2); // Takes a string slice because we don't want to take ownership of the parameter. We want to be able to use the parameter afterwards
        println!("s2 is {s2}");
        let mut s3 = String::from("lo");
        s3.push('l');
        println!("s = {s}, s3 = {s3}");
    }

    println!("==================================================");
    println!("Chapter 8: + operator");
    println!("==================================================");
    {
        let s1 = String::from("hello, ");
        let s2 = String::from(" world");
        let s3 = s1 + &s2; // s1 is moved here so it is no longer valid after this line

        // println!("s1 = {s1}");
        println!("s3 = {s3}, s2 = {s2}");
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3; // Okay all variables to the right of s1 have to be references, becuase the order of operations moves left to right
        println!("s = {s}");
        println!("s2 = {s2}");
        println!("s3 = {s3}");
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s4 = format!("{s1}-{s2}-{s3}");
        println!("s4 = {s4}");
    }
    println!("==================================================");
    println!("Chapter 8: hash map");
    println!("==================================================");
    {
        let map = HashMap::new();
        map.insert("Blue", 10);
        map.insert("Yellow", 50);
        let team_name = String::from("Blue");
        let score = map.get(&team_name).copied().unwrap_or(0);
        // get's the score by passing string slice, a reference because we don't want to own the parameter
        // get retruns an option because there is a chance that we won't find what we are looking for in the map
        // So we get Option<&i32>
        // copied() because we want Option<i32>
        // Unwrap so we get the value, pass the value that we get if None is returned.
    }
    println!("==================================================");
    println!("Chapter 8: iterating");
    println!("==================================================");
    {
        let map = HashMap::new();
        map.insert("Blue", 10);
        map.insert("Yellow", 50);
        for (key, value) in &map {
            println!("{key} = {value}");
        }
    }
    
}
