#![allow(dead_code)]
#![crate_type = "bin"]
#![crate_name = "attributes"]
fn foo() {}

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

#[cfg(enable_bar)]
fn bar() {
    println!("bar got called!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    // runtime check (can be optimized by DCE).
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    // -> rustc main.rs --cfg enable_bar
    if cfg!(enable_bar) {
        bar();
    }
}
