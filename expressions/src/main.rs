fn main() {
    let x = 5;
    x;  // no effect.
    let y = { 1 };  // block with return value.
    println!("{}", y);

    /* if/else */
    if (x == 5) {
        println!("{}", x);
    }
    if x == 5 {
        println!("{}", x);
    }

    /* loop */
    let mut count = 0u32;
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    /* Nesting and labels */
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;  // break the outer loop.
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    /* Return from loops */
    let mut counter = 0;
    let mut result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // break with return value.
        }
    };
    assert_eq!(result, 20);

    /* while */
    // cannot not be broken;
    while result < 10 {
        result -= 1;
    }

    /* for loops */
    // can be used to iterate through an Iterator.
    for x in 1..5 {  // [1, 5).
        println!("x = {}", x);
    }
    for x in 1..=5 {  // [1, 5].
        println!("x = {}", x);
    }

    /* for and iterators */
    #[derive(Debug)]
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {  /// immutable reference. 
        match *name {
            "Bob" => {
                println!("This is Bob!");
            },
            _ => (),
        }
    }
    for name in names.iter_mut() {
        match name {
            name @ &mut "Bob" => {
               *name = "Handsome Bob"; 
            },
            _ => (),
        }
    }
    for name in names.into_iter() {  // move with ownership.
        println!("{}", &name);
    }
    // println!("{:?}", names);  // panic!

    /* match */
    let number = 1;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    /* destructure tuples */
    let triple = (0, -2, -3);
    println!("Tell me about {:?}", triple);
    match triple {
        (x, y, _) => println!("{} {}", x, y),
    }

    /* destructure enums */
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
    }
    let color = Color::RGB(1, 2, 4);
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(ref r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
    }

    /* destructure pointers/ref */
    let ref reference = 4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    let _not_a_reference = 3;
    let ref _is_a_reference = 3;  // create a reference (same as "&").
    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r), 
    }
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
 
    /* destructure structs */
    struct Foo {
        x: (u32, u32),
        y: u32,
        z: u32,
    }
    let foo = Foo { x: (1, 2), y: 3, z: 5 };
    match foo {
        Foo { x: (x1, y1), y, z: 4 } => {
            println!("{} {} {}", x1, y1, y);
        },
        Foo { y, .. } => {
            println!("{}", y);
        },
    }

    /* Guards */
    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"),
    }

    /* Binding */
    fn age() -> u32 {
        15
    }
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1 ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
    fn some_number() -> Option<u32> {
        Some(42)
    }
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }

    /* if-let */
    let some_number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    if let Some(i) = some_number {
        println!("Matched {:?}!", i);
    }
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    enum EnumFoo {
        Bar,
        Baz,
        Qux(u32),
    }
    let a = EnumFoo::Bar;
    let b = EnumFoo::Bar;
    let c = EnumFoo::Qux(100);
    if let EnumFoo::Bar = a {
        println!("a is foobar");
    }
    if let EnumFoo::Bar = b {
        println!("b is foobar");
    }
    if let EnumFoo::Qux(ref value) = c {
        println!("c is {}", value);
    }
    if let EnumFoo::Qux(ref value @ 100) = c {
        println!("c is one hundred");
    }
    
    enum BarFoo { Bar }
    let a = BarFoo::Bar;
    if let BarFoo::Bar = a {
        println!("a is foobar");
    }

    /* while-let */
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
