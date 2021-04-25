// struct Sheep {
//     naked: bool,
//     name: &'static str,
// }

// trait Animal {
//     fn new(name: &'static str) -> Self;
//     fn name(&self) -> &'static str;
//     fn noise(&self) -> &'static str;
//     fn talk(&self) {
//         println!("{} says {}", self.name(), self.noise());
//     }
// }

// impl Sheep {
//     fn is_naked(&self) -> bool {
//         self.naked
//     }
//     fn shear(&mut self) {
//         if self.is_naked() {
//             // implementor methods can use the implementor's trait methods.
//             println!("{} is already naked...", self.name());
//         } else {
//             println!("{} gets a haircut!", self.name);
//             self.naked = true;
//         }
//     }
// }

// impl Animal for Sheep {
//     // `Self` is the implementor type: `Sheep`.
//     fn new(name: &'static str) -> Self {
//         Self {
//             name: name,
//             naked: false,
//         }
//     }

//     fn name(&self) -> &'static str {
//         self.name
//     }

//     fn noise(&self) -> &'static str {
//         if self.is_naked() {
//             "baaaaah?"
//         } else {
//             "baaaaah!"
//         }
//     }

//     // default trait methods can be overridden.
//     fn talk(&self) {
//         // for example, we can add some quiet contemplation.
//         println!("{} pauses briefly... {}", self.name, self.noise());
//     }
// }

#[derive(PartialEq, PartialOrd, Default, Debug)]
struct Centimeters(f64);

struct Sheep {}
struct Cow {}
trait Animal {
    fn noise(&self) -> &'static str;
}
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

/* Operator Ovberloading */
use std::ops;
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;
#[derive(Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called.");
        FooBar
    }
}
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called.");
        BarFoo
    }
}

/* Drop */
struct Droppable {
    name: &'static str,
}
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

/* Iterators */
struct Fibonacci {
    curr: u32,
    next: u32,
}
impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

/* impl Trait */
use std::iter;
use std::vec::IntoIter;

fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// same as above.
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

/* Clone */
#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

/* Supertraits */
trait Person {
    fn name(&self) -> String;
}
trait Student: Person {
    fn university(&self) -> String;
}
trait Programmer {
    fn fav_language(&self) -> String;
}
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

/* Disambiguating overlapping traits */
trait UsernameWidget {
    fn get(&self) -> String;
}
trait AgeWidget {
    // get the selected age out of this widget.
    fn get(&self) -> u8;
}
// a form with both a UsernameWidget and an AgeWidget.
struct Form {
    username: String,
    age: u8,
}
impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}
impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    // let mut dolly: Sheep = Animal::new("Dolly");
    // dolly.talk();
    // dolly.shear();
    // dolly.talk();

    let x: Centimeters = Default::default();
    let y = Centimeters(20f64);
    println!("{:?} {}", x, x < y);

    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );

    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);

    let _a = Droppable { name: "a" };
    // block A.
    {
        let _b = Droppable { name: "b" };
        // block B.
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just exited block B");
        println!("Exiting block A");
    }
    println!("Just exited block A");
    // variable can be manually dropped using the `drop` function.
    drop(_a);
    println!("end of the main function");

    let mut sequence = 0..3; // an "Iterator".
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    for i in 0..3 {
        println!("> {}", i);
    }
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    let unit = Unit;
    let copied_unit = unit; // copied.
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);
    let cloned_pair = moved_pair.clone();
    drop(moved_pair);
    println!("clone: {:?}", cloned_pair);

    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
