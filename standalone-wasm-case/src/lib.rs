#[link(wasm_import_module = "./mod.js")]
extern { 
  fn fooA(); 
  fn fooB();
}

// export a Rust function called `bar`.
#[no_mangle]
pub extern fn bar(x: u32, y: u32) -> u32 {
    unsafe { fooA(); fooB(); }
    x + y
}

// setup a custom section.
#[link_section = "hello"]
pub static SECTION: [u8; 24] = *b"This is a custom section";
