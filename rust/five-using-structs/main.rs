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
        };

        println!("the area of the rectangle is {} square pixels.", area3(&rect));
    }

    {
        println!("==================================================");
        println!("Chapter 5: Example Program - Debugging");
        println!("==================================================");
        
        let rect1 = Rectangle {
            width: 30,
            height: 50
        };

        println!("rect1 is {:?}", rect1);
    }

    {
        println!("==================================================");
        println!("Chapter 5: Example Program - dbg!");
        println!("==================================================");
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50
        };

        dbg!(&rect1);
    }

    {
        println!("==================================================");
        println!("Chapter 5: Method syntax");
        println!("==================================================");
        #[derive(Debug)]
        struct Rectangle2 {
            width: u32,
            height: u32,
        }

        impl Rectangle2 {
            fn area(&self) -> u32{
                self.width * self.height
            }
        }

        let rect1 = Rectangle2 {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    {
        println!("==================================================");
        println!("Chapter 5: Taking mutable ownership from a method");
        println!("==================================================");
        struct Race {
            name: String,
            laps: Vec<i32>,
        }

        impl Race {
            fn new(name: &str) -> Self {
                Self { name: String::from(name), laps: Vec::new() } // No receiver, a static method
            }

            fn add_lap(&mut self, lap: i32) { // Exclusive borrowed read-write access to self
                self.laps.push(lap);
            }

            fn print_laps(&self) { // Shared and read-only borrowed access to self
                println!("Recorded {} laps for {}:", self.laps.len(), self.name);
                for (idx, lap) in self.laps.iter().enumerate() {
                    println!("Lap {idx}: {lap} sec");
                }
            }

            fn finish(self) { // Exclusive ownership of self
                let total = self.laps.iter().sum::<i32>();
                println!("Race {} is finished, total lap time: {}", self.name, total); // Doesn't return ownership, memory dopped on name and vec, race out of scope
            }
        }

        let mut race = Race::new("Monaco Grand Prix");
        race.add_lap(70);
        race.add_lap(68);
        race.print_laps();

        race.add_lap(71);
        race.print_laps();

        race.finish();
        // race.add_lap(42); doesn't work
    }

    {
        println!("==================================================");
        println!("Chapter 5: More parameters");
        println!("==================================================");
        #[derive(Debug)]
        struct Rectangle2 {
            width: u32,
            height: u32,
        }

        impl Rectangle2 {
            fn area(&self) -> u32{
                self.width * self.height
            }

            fn can_hold(&self, other: &Rectangle2) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let rect1 = Rectangle2 {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle2 {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle2 {
            width: 60,
            height: 45,
        };
    
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect3.can_hold(&rect3));
    }
    {
        println!("==================================================");
        println!("Chapter 5: associated functions");
        println!("==================================================");
        struct Rectangle2 {
            width: u32,
            height: u32,
        }
        struct Rectangle3 {
            width: u32,
            height: u32,
        }

        impl Rectangle2 {
            fn square(size: u32) -> Self {
                Self { width: size, height: size, }
            }
        }

        impl Rectangle3 {
            fn square(size: u32) -> Rectangle3 {
                Rectangle3 { width: size, height: size, }
            }
        }

        let sqr1 = Rectangle2::square(3);
        let sqr2 = Rectangle3::square(3);
    
        println!("sqr1 width = {}, sqr1 height = {}", sqr1.width, sqr1.height);
        println!("sqr2 width = {}, sqr2 height = {}", sqr2.width, sqr2.height);
    }
}
#[derive(Debug)]
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
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}