fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    let default_float = 3.0;
    let default_integer = 7;
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;
    let mut mutable = 12;
    mutable = 21;
    let mutable = true; // overwritten with shadowing.

    /* Literals and Operators */
    println!("{}", 1_000);
    println!("{}", 0.000_001);
    println!("{}", 0b0011u32 & 0b0101);
    println!("{}", 1u32 << 5);
    println!("{}", false || true);

    /* Tuple */
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;
        (boolean, integer)
    }
    let t = reverse((1, true));
    println!("{:?}", &t);
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);  // cannot be printed.
    // println!("too long tuple: {:?}", too_long_tuple);

    // the comma is required to tell them apart from a literal surrounded by parentheses.
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    /* Array and Slices */
    use std::mem;
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    // Arrays are stack allocated.
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    analyze_slice(&xs);
    // slice section -> [).
    analyze_slice(&ys[1..4]);
}
