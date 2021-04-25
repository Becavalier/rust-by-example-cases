fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;
    let _unused_variable = 3u32;
    let noisy_unused_varible = 2u32;

    /* Mutability */
    let mut mutable_binding = 1;
    mutable_binding += 1;
    println!("{}", mutable_binding);

    /* Scope and Shadowing */
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("{}", short_lived_binding);
        let long_lived_binding = 10; // scoped shadowing.
        println!("{}", long_lived_binding);
    }
    println!("{}", long_lived_binding);
    let long_lived_binding = 20;
    println!("{}", long_lived_binding);

    /* Declare */
    let a_binding; // uninitialized stack memory.
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("{}", a_binding);

    /* Freezing */
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer; // freeze the variable.
        println!("{}", _mutable_integer);
        let mut mutable_integer = _mutable_integer;
        println!("{}", mutable_integer);
    }
    _mutable_integer = 10;
    println!("{}", _mutable_integer);
}
