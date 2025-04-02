use std::ops::Deref;
use std::rc::Rc;
use std::cell::Cell;

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
    
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    let e = CustomSmartPointer {
        data: String::from("more stuff"),
    };
    println!("Created another custom smart pointer.");
    drop(e);
    println!("Dropping the new CustomSmartPointer before the end of main.");

    {
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
        println!("Count after creating a = {}", Rc::strong_count(&a));
        let _b = List::Cons(3, Rc::clone(&a));
        println!("Count after creating b = {}", Rc::strong_count(&a));
        {
            let _c = List::Cons(4, Rc::clone(&a));
            println!("Count after creating c = {}", Rc::strong_count(&a));
        }
        println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    {
        fn foo(cell: &Cell<u32>) {
            let value = cell.get();
            cell.set(value * 2);
        }

        let cell = Cell::new(0);
        let value = cell.get();
        println!("the value of value: {}", value);
        let new_value = cell.get() + 1;
        println!("the value of new_value: {}", new_value);
        foo(&cell);
        println!("the value of cell after foo: {}", cell.get());
        cell.set(new_value);
        println!("the value of cell after cell.set(new_value): {}", cell.get());
    }
}
