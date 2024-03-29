use std::fmt::Display;

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
    println!("==================================================");
    println!("Chapter 10: implementing traits on types");
    println!("==================================================");
    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }
        pub struct Tweet{
            pub username: String,
            pub content: String,
            pub reply: String,
            pub retweet: String,
        }
        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }
        impl Summary for Tweet{
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }
        let article = NewsArticle {
            headline: String::from("Testing Traits"),
            location: String::from("New York, NY"),
            author: String::from("Paul Nakonieczny"),
            content: String::from("This is an article about testing traits on types."),
        };
        let tweet = Tweet {
            username: String::from("sgt.forge"),
            content: String::from("this is a cool tweet"),
            reply: String::from("reply"),
            retweet: String::from("retweet"),
        };
        println!("{}", article.summarize());
        println!("{}", tweet.summarize());
    }
    println!("==================================================");
    println!("Chapter 10: traits as parameters");
    println!("==================================================");
    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        pub fn notify(item: &impl Summary) {
            println!("Breaking new! {}", item.summarize());
        }
        
    }
    println!("==================================================");
    println!("Chapter 10: traits as parameters");
    println!("==================================================");
    {
        struct Pair<T> {
            x: T,
            y: T,
        }
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self. y {
                    println!("The lartgest member is x = {}", self.x);
                } else {
                    println!("The lartgest member is y = {}", self.y);
                }
            }
        }
    }
    println!("==================================================");
    println!("Chapter 10: validating references with lifetimes");
    println!("==================================================");
    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let z;
        {
            let x = "hihihiihihihihhihih";
            let y = "hello";
            z = longest(x, y);
        }
        
        println!("the longest = {}", z);
    }
    println!("==================================================");
    println!("Chapter 10: playing with lifetimes");
    println!("==================================================");
    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        fn test_lifetimes() -> &'static str {
            let ret = "something else";
            &ret
        }
        fn test2(s1: &str, s2: &str) -> &'static str{
            println!("s1 = {s1}, s2 = {s2}");
            let some_str = "a string literal";

            &some_str
        }

        let a1 = "hi";
        let a2 = "test";
        let a3 = test_lifetimes();
        let a4 = test2(a1, a2);
        println!("a3 = {a3}");
        println!("a4 = {a4}");
    }

}

fn largest_copy(list: &[i32]) -> i32 {
    let mut largest: i32 = list[0];

    for item in list {
        if item > &largest {
            largest = *item;
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