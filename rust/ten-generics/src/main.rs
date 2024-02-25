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