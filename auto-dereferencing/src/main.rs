struct X { val: i32 }
impl std::ops::Deref for X {
    type Target = i32;
    fn deref(&self) -> &i32 { &self.val }
}

trait M { fn m(self); }
impl M for i32   { fn m(self) { println!("i32::m()");  } }
impl M for X     { fn m(self) { println!("X::m()");    } }
impl M for &X    { fn m(self) { println!("&X::m()");   } }
impl M for &&X   { fn m(self) { println!("&&X::m()");  } }
impl M for &&&X  { fn m(self) { println!("&&&X::m()"); } }

trait RefM { fn refm(&self); }
impl RefM for i32  { fn refm(&self) { println!("i32::refm()");  } }
impl RefM for X    { fn refm(&self) { println!("X::refm()");    } }
impl RefM for &X   { fn refm(&self) { println!("&X::refm()");   } }
impl RefM for &&X  { fn refm(&self) { println!("&&X::refm()");  } }
impl RefM for &&&X { fn refm(&self) { println!("&&&X::refm()"); } }

struct Y { val: i32 }
impl std::ops::Deref for Y {
    type Target = i32;
    fn deref(&self) -> &i32 { &self.val }
}

// &Z -> &Y -> val: i32.
struct Z { val: Y }
impl std::ops::Deref for Z {
    type Target = Y;
    fn deref(&self) -> &Y { &self.val }
}

#[derive(Clone, Copy)]
struct A;

impl M for    A { fn m(self) { println!("A::m()");    } }
impl M for &&&A { fn m(self) { println!("&&&A::m()"); } }

impl RefM for    A { fn refm(&self) { println!("A::refm()");    } }
impl RefM for &&&A { fn refm(&self) { println!("&&&A::refm()"); } }

fn main() {
    let x = X { val: 10 };
    println!("{}", *x);  // 10.

    (*X { val: 10 }).m();
    X { val: 10 }.m();
    (&X { val: 10 }).m();
    (&&X { val: 10 }).m();
    (&&&X { val: 10 }).m();
    (&&&&X { val: 10 }).m();
    (&&&&&X { val: 10 }).m();
    (&&&&&&X { val: 10 }).m();

    (*X { val: 10 }).refm();
    X { val: 10 }.refm();
    (&X { val: 10 }).refm();
    (&&X { val: 10 }).refm();
    (&&&X { val: 10 }).refm();
    (&&&&X { val: 10 }).refm();
    (&&&&&X { val: 10 }).refm();
    (&&&&&&X { val: 10 }).refm();

    Y { val: 10 }.refm();
    Z { val: Y { val: 10 } }.refm();

    A.m();
    (&A).m();
    (&&A).m();
    (&&&A).m();
    (&&&&A).m();

    A.refm();
    (&A).refm();
    (&&A).refm();
    (&&&A).refm();
    (&&&&A).refm();
}
