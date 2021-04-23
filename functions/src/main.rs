fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
impl Point {
    // associated functions (static functions).
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
impl Rectangle {
    // "&self" is sugar for "self: &Self".
    fn area(self: &Self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
    fn translate(self: &mut Self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}
struct Pair(Box<i32>, Box<i32>);
impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        drop(first);
        drop(second);
        // or `first` and `second` will go out of scope and get freed.
    }
}
fn foo(f: fn(i32) -> i32) {
    f(10);
}
fn zoo(f: fn()) {
    f();
}
fn bar(x: i32) -> i32 {
    x
}
fn apply<F>(f: F) where F: FnOnce () {
    f();
}
fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}
fn call_me<F: Fn()>(f: F) {
    f();
}
fn my_function() {
    println!("I'm a function!");
}
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || {
        println!("This is a: {}", text)
    }
}
fn create_fnmut() -> impl FnMut() {
    let mut text = "FnMut".to_owned();
    move || {
        text.push_str(" !!!");
        println!("This is a: {}", text)
    }
}
fn create_fonce() -> impl FnOnce() {
    let mut text = "FnOnce".to_owned();
    move || {
        text.push_str(" !!!");
        println!("This is a: {}", text);
    }
}
fn print_coordinates(&(x, y): &(i32, i32)) { 
    println!("Current location: ({}, {})", x, y);
}
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
fn main() {
    fizzbuzz_to(100);
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1.0, 1.0);
    println!("{:#?}", square);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    /* Closures */
    fn function(i: i32) -> i32 { i + 1 }
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;
    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    let one = || i;
    println!("closure returning one: {}", one());
    let fp: fn(i32) -> i32 = closure_inferred;
    println!("{:?}", fp as i32);  // function pointer.
    foo(bar);
    foo(|x| x);  // closure can be passed through another function call.

    /* Capturing */
    use std::mem;
    let color = String::from("green");
    let print = || println!("`color`: {}", color);  // &T.
    print();
    let _reborrow = &color;
    print();
    let _color_moved = color; // take ownership.
    let mut count = 0;
    let mut inc = || {
        count += 1;  // &mut T.
        println!("`count`: {}", count);
    };
    inc();
    // let _reborrow = &count; // panic!
    inc();
    let _count_reborrowed = &mut count; 
    // a non-copy type.
    let movable = Box::new(3);
    let consume = || {  // T.
        println!("`movable`: {:?}", movable);
        println!("{}", _color_moved);  // &T;
        mem::drop(movable);
    };
    println!("{}", _color_moved);
    consume();

    let haystack = vec![1, 2, 4];
    let contains = move |needle| haystack.contains(needle);  // T.
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    /* As input parameters */
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        println!("I said {}.", greeting);  // captured by reference.
        farewell.push_str("!!!");  // mutable reference.
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");
        mem::drop(farewell);  // captured by value.
    };
    apply(diary);
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
    foo(double);
    // only closures which do not capture outer values can be passed through function pointer.
    // zoo(diary);

    /* Type anonymity */
    /* Input functions */
    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(my_function);
    println!("{}", my_function as u32);  // take function pointer.

    /* As output parameters */
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fonce();
    fn_plain();
    fn_mut();
    fn_once();

    /* Iterator::any */
    let vec1 = vec![1, 2, 4];
    let vec2 = vec![4, 5, 6];
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));  // value wiil be moved.
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
    println!("{:?}", array2);
    println!("{:?}", vec1);

    /* Searching through iterators */
    let vec3 = vec![1, 2, 4];
    let vec4 = vec![4, 5, 6];
    let mut iter = vec3.iter();
    let mut into_iter = vec4.into_iter();
    println!("Find 2 in vec3: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec4: {:?}", into_iter.find(|&x| x == 2));
    println!("{:?}", vec3);
    // println!("{:?}", vec4);  // panic.
    let array3 = [1, 2, 3];
    let array4 = [4, 5, 6];
    println!("Find 2 in array3: {:?}", array3.iter().find(|&&x| x == 2));
    println!("Find 2 in array4: {:?}", array4.into_iter().find(|&&x| x == 2));
    let index_of_first_even_number = vec3.iter().position(|x| x % 2 == 0);
    if let Some(p) = index_of_first_even_number {
        println!("{}", p);
    }
    let point = (3, 5); 
    print_coordinates(&point);  // pattern matching.

    /* Higher Order Functions */
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)
            .take_while(|&n_squared| n_squared < upper)
            .filter(|&n_squared| is_odd(n_squared))
            .fold(0, |acc, n_squared| acc + n_squared);
    println!("functional style: {}", sum_of_squared_odd_numbers);

    /* Diverging functions */
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue,  // -> ! can match everything.
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
