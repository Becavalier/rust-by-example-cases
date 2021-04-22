
// used for fallible conversions.
use std::convert::{TryFrom, TryInto};
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);
impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() -> Result<(), <EvenNumber as TryFrom<i32>>::Error> {
    /* From */
    use std::convert::From;
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    let num = Number::from(30);
    println!("{:?}", num);

    /* Into */
    let int = 5;
    let num: Number = int.into();  // will call Number::from implicitly.
    println!("{:?}", num);

    /* TryFrom and TryInto */
    let x = EvenNumber::try_from(10)?;
    println!("x = {:?}", x);
    
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(1), Err(()));
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    /* To and from Strings */
    struct Circle {
        radius: i32,
    }
    use std::fmt;
    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());  // struct instance to String.

    // String to the numeric value (based on the "FromStr" trait).
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    // default return value.
    Ok(())
}
