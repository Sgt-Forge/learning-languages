fn main() {
    println!("==================================================");
    println!("For Loops");
    println!("==================================================");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    println!("==================================================");
    println!("For Loops - Countdown");
    println!("==================================================");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}