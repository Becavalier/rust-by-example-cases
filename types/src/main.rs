#![allow(overflowing_literals)]
fn main() {
    /* Casting */
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    println!("{}", 1000 as u8);
    println!("{}", -1i8 as u8);
    println!("{}", 1000 % 256);
    // since Rust 1.45, the `as` keyword performs a *saturating cast* -
    // when casting from float to int.
    println!("{}", 300.0_f32 as u8); // 255.
    println!("-100.0 as u8 is {}", -100.0_f32 as u8); // 0.
    println!("{}", f32::NAN as u8); // 0.
    unsafe {
        // rounds toward zero and converts to any primitive integer type, -
        // assuming that the value is finite and fits in that type.
        // !!! the results might overflow (against *saturating cast*), plus minor overhead !!!
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }

    /* Literals */
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    let i = 1; // i32.
    let f = 1.0; // f64.
    println!("{}", std::mem::size_of_val(&x));
    println!("{}", std::mem::size_of_val(&y));
    println!("{}", std::mem::size_of_val(&z));
    println!("{}", std::mem::size_of_val(&i));
    println!("{}", std::mem::size_of_val(&f));

    /* Inference */
    let elem = 5u8;
    let mut vec = Vec::new(); // the type of the holding value will be inferred.
    vec.push(elem);
    println!("{:?}", vec);

    /* Aliasing */
    type NanoSecond = u64;
    type Inch = u64;
    let nanoseconds: NanoSecond = 6 as u64;
    let inches: Inch = 2 as u64;
    println!("{} {}", nanoseconds, inches);
}
