enum Status {
    Rich,
    Poor,
}
enum Work {
    Civilian,
    Soldier,
}

fn main() {
    /* Struct */
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    struct Unit;
    struct Pair(i32, f32);
    struct Point {
        x: f32,
        y: f32,
    }
    struct Rectangle {
        top_left: Point,
        bottom_left: Point,
    }
    let p = Person {
        name: String::from("Jason"),
        age: 19,
    };
    println!("{:#?}", p);

    // make a new Person by using struct update syntax to use the fields of the other one.
    let another_p = Person {
        name: String::from("Alice"),
        ..p
    };
    println!("{:#?}", another_p);

    // destructure a struct.
    let Person { name, age } = p;
    println!("{} {}", name, age);

    // unit and tuple struct.
    let _unit = Unit; // ZST.
    let pair = Pair(1, 1.1);
    println!("{}, {}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;
    println!("{}, {}", integer, decimal);

    /* Enum */
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
        }
    }
    inspect(WebEvent::Click { x: 1, y: 2 });

    /* Type Aliases */
    #[derive(Debug)]
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
    println!("{:?}", Operations::Add);
    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    use crate::Status::{Poor, Rich};
    use crate::Work::*;
    let status = Poor;
    match status {
        Poor => println!("The rich have lots of money!"),
        _ => (),
    }

    /* C-like Enum */
    enum Number {
        Zero,
        One,
        Two,
    }
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    println!("zero is {}", Number::Zero as i32);
    println!("roses are #{:06x}", Color::Red as i32);

    /* Linked-list */
    enum List {
        Cons(u32, Box<List>),
        Nil,
    }
    use List::*;
    impl List {
        fn new() -> List {
            Nil
        }
        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self)) // move to heap.
        }
        // with the help of RFC 2005-match-ergonomics.
        fn len(&self) -> u32 {
            match self {
                Cons(_, tail) => 1 + tail.len(), // recursive.
                Nil => 0,
            }
        }
        // fn len(&self) -> u32 {
        //     match *self {
        //         Cons(_, ref tail) => 1 + tail.len(),  // recursive.
        //         Nil => 0,
        //     }
        // }
        // fn len(&self) -> u32 {
        //     match self {
        //         &Cons(_, ref tail) => 1 + tail.len(),  // recursive.
        //         &Nil => 0,
        //     }
        // }
        fn stringify(&self) -> String {
            match self {
                Cons(head, tail) => {
                    // take a reference of tail.
                    format!("{}, {}", head, tail.stringify())
                }
                Nil => {
                    format!("Nil")
                }
            }
        }
    }
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    /* Constants */
    static mut LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;
    unsafe {
        LANGUAGE = "C++";
        println!("{}", LANGUAGE);
    }
}
