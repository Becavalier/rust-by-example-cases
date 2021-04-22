fn main() {
    let x = String::from("world!");
    let s = format!("Hello, {}", &x);
    print!("{}", s);
    println!("{}", s);  // with newline, to io::stdout.
    eprint!("{}", s);
    eprintln!("{0}", s);  // use the first value.
    println!("{:b}", 100);  // with formatting flags (binary).
    println!("{}", 100u32);
    
    // use named arguments.
    println!("{subject} {verb} {object}.", 
        object = "the lazy dog", 
        subject = "the quick brown fox",
        verb = "jumps over");
    
    // align to right, padding with zero numerical extension.
    println!("{number:>width$} {number:<0width$}",
        number = 1,
        width = 6);
    
    // print compound types with Debug trait.
    #[derive(Debug)]
    struct SA(i32); 
    println!("{:#?}", &SA(10));

    // print compound types with custom Display trait.
    use std::fmt;
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    impl<'a> Person<'a> {
        fn new(name: &'a str, age: u8) -> Self {
            Person {
                name, age,
            }
        }
    }
    impl<'a> fmt::Display for Person<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {}", &self.name, &self.age)
        }
    }
    println!("{}", &Person::new("Jason", 19));
    println!("{:#?}", &Person::new("Alice", 21));

    // implementing Display trait for Vec.
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;  // propagate the error to the outer scope if any.
            for (index, &v) in vec.iter().enumerate() {
                if index == 0 { 
                    write!(f, "{}", v)?;
                } else {
                    write!(f, ",{}", v)?;
                }
            }
            write!(f, "]")
        }
    }
    println!("{}", &List(vec![1, 2, 3]));

    // formatting with macro format!.
    let foo = 3735928559i64;
    println!("{}", format!("{}", foo));
    println!("{}", format!("0x{:X}", foo));
    println!("{}", format!("b'{:b}", foo));
    println!("{}", format!("0o{:o}", foo));

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }
    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }
    for city in [  // pattern matching.
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 }, 
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
    ].iter() {
        println!("{}", city);
    }
}
  