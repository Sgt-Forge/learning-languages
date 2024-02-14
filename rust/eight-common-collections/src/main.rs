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
}
