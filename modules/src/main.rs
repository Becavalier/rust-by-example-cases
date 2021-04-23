mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }
    pub fn function() {
        println!("called `my_mod::function()`");
    }
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }
        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }
        pub(in crate::my_mod) fn public_function_in_my_mod() {}
        // only visible within the current module, which is the same as leaving them private.
        pub(self) fn public_function_in_nested() {}
        // only visible within the parent module.
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }
    // visible only within the current crate.
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
        // private parent items will still restrict the visibility of a child item, -
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }
    pub struct ClosedBox<T> {
        contents: T, // private.
    }
    impl<T> ClosedBox<T> {
        // a public constructor method.
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
    fn function() {}
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    pub fn indirect_call() {
        self::function();  // the function with this mod.
        function();
        self::cool::function();
        super::function();
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

use deeply::nested::function as other_function;

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

fn main() {
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be -
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate.
    my_mod::public_function_in_crate();
    other_function();
    {
        // this `function()` will shadow the outer one.
        use crate::deeply::nested::function;
        function();
    }
}
