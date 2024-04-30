use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y); compile time error because MyBox cannot be 
                        // dereferenced
    
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    assert_eq!(5, x);
    assert_eq!(5, *y); // compiles because we implemented deref
}
