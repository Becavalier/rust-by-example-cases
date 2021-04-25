use std::num::ParseIntError;

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }
    println!("Some refreshing {} is all I need.", beverage);
}
fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}
fn give_royal(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAaaaaa!!!!");
    }
    println!("I love {}s!!!!!", inside);
}
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age?;
    Some(format!("Next year I will be {}", next_age))
}
struct Person {
    job: Option<Job>,
}
#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}
#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}
impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}
#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number + second_number
}
/* Option<T, U>::and_then */
fn sq(x: u32) -> Option<u32> {
    Some(x * x)
}
fn nope(_: u32) -> Option<u32> {
    None
}

// define a generic alias for a `Result`.
type AliasedResult<T> = Result<T, ParseIntError>;
fn multiply_with_map(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

/* Early returns */
fn multiply_with_early_returns(
    first_number_str: &str,
    second_number_str: &str,
) -> AliasedResult<i32> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number) => first_number,
        Err(e) => return Err(e), // early return.
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number) => second_number,
        Err(e) => return Err(e),
    };
    Ok(first_number * second_number)
}
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/* ? */
fn multiply_with_question_mark(
    first_number_str: &str,
    second_number_str: &str,
) -> AliasedResult<i32> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

/* Handle mixed error types */
fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
    // Option<Result<i32, ParseIntError>> -> Result<Option<i32>, ParseIntError>.
    opt.map_or(Ok(None), |r| r.map(Some)) // `r.map(Some)` will be the return value.
}

/* Defining an error type */
use std::fmt;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double.")
    }
}
fn double_first_x(vec: Vec<&str>) -> std::result::Result<i32, DoubleError> {
    vec.first()
        // change the error to our new type.
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // update to the new error type here also.
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

/* Boxing errors */
use std::error;

#[derive(Debug, Clone)]
struct EmptyVec;
impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double.")
    }
}
impl error::Error for EmptyVec {}
fn double_first_box(vec: Vec<&str>) -> std::result::Result<i32, Box<dyn error::Error>> {
    vec.first()
        // lazily evaluated, compared to `ok_or()`.
        .ok_or_else(|| EmptyVec.into()) // converts to Box, move value to heap.
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // converts to Box.
                .map(|i| 2 * i)
        })
}

/* Other uses of ? */
#[derive(Debug)]
struct EmptyVecB;
impl fmt::Display for EmptyVecB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double.")
    }
}
impl error::Error for EmptyVecB {}
fn double_first_question_box(vec: Vec<&str>) -> std::result::Result<i32, Box<dyn error::Error>> {
    // use `?` to get the inner value out immediately.
    // use ? where the error is convertible to the return type, it will convert automatically.
    // `Box::<EmptyVec>::from(EmptyVec)`.
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

/* Wrapping errors */
use std::error::Error as _;
#[derive(Debug)]
enum DoubleErrorB {
    EmptyVec,
    Parse(ParseIntError),
}
impl fmt::Display for DoubleErrorB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleErrorB::EmptyVec => write!(f, "please use a vector with at least one element"),
            DoubleErrorB::Parse(ref e) => {
                write!(f, "the provided string could not be parsed as int")
            }
        }
    }
}
impl error::Error for DoubleErrorB {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleErrorB::EmptyVec => None,
            DoubleErrorB::Parse(ref e) => Some(e),
        }
    }
}
impl From<ParseIntError> for DoubleErrorB {
    fn from(err: ParseIntError) -> DoubleErrorB {
        DoubleErrorB::Parse(err)
    }
}
fn double_first_with_wrapping(vec: Vec<&str>) -> std::result::Result<i32, DoubleErrorB> {
    let first = vec.first().ok_or(DoubleErrorB::EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn main() -> Result<(), ParseIntError> {
    drink("water");
    // drink("lemonade");

    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    // let nothing = None;

    give_royal(bird);
    // give_royal(nothing);

    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);

    assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
    assert_eq!(Some(2).and_then(sq).and_then(nope), None);

    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // let tt = multiply("t", "2");  // panic.
    // println!("double is {}", tt);
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };

    print(multiply_with_early_returns("10", "2"));
    print(multiply_with_early_returns("t", "2"));

    let strings = vec!["tofu", "93", "18"];
    println!("{:?}", double_first_x(strings));

    /* Iterating over Results */
    let strings = vec!["tofu", "93", "18"];

    let numbers: Vec<_> = strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);

    let numbers: Result<Vec<_>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);

    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
    // `collect()` - transforms an iterator into a collection.
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    Ok(())
}
