use std::fs::File;

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
        }
    }
}
