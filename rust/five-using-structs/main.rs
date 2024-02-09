fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    {
        println!("==================================================");
        println!("Chapter 5: structs");
        println!("==================================================");
        let mut user1 = User {
            email: String::from("someone@example.com"),
            active: true,
            username: String::from("someone"),
            sign_in_count: 1
        };

        let user2 = User {
            email: String::from("different@example.com"),
            ..user1 // move username filed, but copy the primitive data types
        };

        // println!("user1.username = {}, user2.username = {}", user1.username, user2.username); this causes an error because username field was moved
        println!("user1.sign_in_count = {}, user2.sign_in_count = {}", user1.sign_in_count, user2.sign_in_count);

        user1.username = String::from("hadtoaddanotherusername");
        println!("user1.username = {}, user2.username = {}", user1.username, user2.username);
    }

    {
        println!("==================================================");
        println!("Chapter 5: tuple structs");
        println!("==================================================");
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let _black = Color(0, 0, 0);
        let _origin = Point(0, 0, 0);
    }

    {
        println!("==================================================");
        println!("Chapter 5: Unit like structs");
        println!("==================================================");
        struct AlwaysEqual;

        let subject = AlwaysEqual;
    }

    {
        println!("==================================================");
        println!("Chapter 5: Example Program - Just variables");
        println!("==================================================");
        let width1 = 30;
        let height1 = 50;

        println!("the area of the rectangle is {} square pixels.", area1(width1, height1));
    }

    {
        println!("==================================================");
        println!("Chapter 5: Example Program - tuples");
        println!("==================================================");
        let rect = (30, 50);

        println!("the area of the rectangle is {} square pixels.", area2(rect));
    }

    {
        println!("==================================================");
        println!("Chapter 5: Example Program - Structs");
        println!("==================================================");
        // struct Rectangle { available outside of main
        //     width: u32,
        //     height: u32
        // }

        let rect = Rectangle {
            width: 30,
            height: 50
        }

        println!("the area of the rectangle is {} square pixels.", area3(rect));
    }
}
struct Rectangle {
    width: u32,
    height: u32
}

// bad code because we don't know the relationship between the parameters
// Little context, you would have to be the author to know what's happening and where these come from
fn area1(width: u32, height: u32) -> u32 {
    width * height
}

// This is getting better, I know that the arguments are the dimenssion of something, but now I don't know which is width, height, etc. they are just numbers
// That might be okay for area, but not for drawing
fn area2(dimensions: (u32, u32)) -> {
    dimensions.0 * dimensions.1
}

fn area2(rect: Rectangle) -> {
    rect.width * rect.height
}