struct A;
struct S(A);
struct SingleGen<T>(T);
fn gen_spec_t(_s: SingleGen<A>) {}
fn generic<T>(_s: SingleGen<T>) {
    println!("generic has been called!");
}
struct GenVal<T> {
    gen_val: T,
}
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}
struct Empty;
struct Null;
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}
// implement `DoubleDrop<T>` for any generic parameter `T` and caller `U`.
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}
// trait bounds.
fn printer<T: std::fmt::Debug>(t: T) {
    println!("{:?}", t);
}
trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: std::fmt::Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}
/* New Type Idiom */
struct Years(i64);
struct Days(i64);
impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}
impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}
fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}
struct Container(i32, i32);
trait Contains {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}
impl Contains for Container {
    type A = i32;
    type B = i32;
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 {
        self.0
    }

    // Grab the last number.
    fn last(&self) -> i32 {
        self.1
    }
}
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}
use std::marker::PhantomData;
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);
#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

pub trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}
#[derive(Debug, Clone, Copy)]
struct Length<T>(f64, PhantomData<T>);
impl<T> std::ops::Add for Length<T> {
    type Output = Length<T>;
    fn add(self, rhs: Length<T>) -> Length<T> {
        // `+` calls the `Add` implementation for `f64`.
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let _char: SingleGen<char> = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
    // explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SingleGen('a'));
    generic(SingleGen('b'));

    let y = GenVal { gen_val: 3i32 };
    println!("{}", y.value());

    let empty = Empty;
    let null = Null;
    let mut x = 10;
    empty.double_drop(null);
    // x could be copied.
    y.double_drop(x);

    // false.
    println!("{}", std::mem::needs_drop::<i32>());

    let vec = vec![1, 2, 3];
    vec.print_in_option();

    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));

    // associated types.
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));

    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // compile-time Error! Type mismatch so these cannot be compared:
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);

    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition works.
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);
}
