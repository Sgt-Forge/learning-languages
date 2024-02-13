use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("==================================================");
    println!("Chapter 7: Managing Growing Projects with Packages, Crates, and Modules");
    println!("==================================================");

    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
}