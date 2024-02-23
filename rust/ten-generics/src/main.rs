fn main() {
    println!("==================================================");
    println!("Chapter 10: Generics");
    println!("==================================================");
    {
        let number_list1 = vec![1, 2, 3, 4, 5];
        let number_list2 = vec![6, 7, 8, 9, 10];
        let largest1 = largest_copy(&number_list1);
        let largest2 = largest_reference(&number_list2);
        println!("largest1 = {:?}", largest1);
        println!("largest2 = {:?}", largest2);
    }
}

fn largest_copy(list: &[i32]) -> i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_reference(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}