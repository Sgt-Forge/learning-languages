use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("==================================================");
    println!("Chapter 9: Errors unrecoverable errors");
    println!("==================================================");
//    {
//        let v = vec![1, 2, 3];
//        let s = v[99]; // panic
//    }
    println!("==================================================");
    println!("Chapter 9: Recoverable errors");
    println!("==================================================");
    {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }
    {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file {:?}", e),
                },
                ohter_error => {
                    panic!("Problem opening the file: {:?}", ohter_error);
                }
            },
        };
    }
    println!("==================================================");
    println!("Chapter 9: Propagating errors");
    println!("==================================================");
    {
        println!("read_username1 = {:?}", read_username1());
        println!("read_username2 = {:?}", read_username2());
        println!("read_username3 = {:?}", read_username3());
    }
}

fn read_username1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt"); 

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
fn read_username2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    let username_file = username_file.read_to_string(&mut username)?;
    Ok(username)
}
fn read_username3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
