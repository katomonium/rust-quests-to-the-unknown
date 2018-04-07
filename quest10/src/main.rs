// Module

mod code;


mod my_mod {

    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    pub fn function() {
        println!("called `my_mod::function()`");
    }

    pub fn indirect_access() {
        println!("called `my_mod::indirect_access()`");
        print!(">   ");
        private_function();
    }

    pub mod nested {
        
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        pub(in my_mod) fn public_function_in_my_mod() {
            println!("called `my_mod::nested::public_function_in_my_mod`");
            print!(">   ");
            public_function_in_nested();
        }
        
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested`");
        }

        pub(in my_mod) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod`");
        }

    }

    pub fn call_public_function_in_my_mod() {
        println!("called `call_public_function_in_my_mod()`");
        print!(">   ");
        nested::public_function_in_my_mod();
        print!(">   ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn publuc_function_in_crate() {
        println!("called `my_mod::publuc_function_in_crate()`");
    }

}


fn function() {
    println!("called `function()`");
}


fn main() {
    function();
    my_mod::function();
    my_mod::nested::function();

    my_mod::indirect_access();
    my_mod::call_public_function_in_my_mod();
    my_mod::publuc_function_in_crate();

    code::flora();
}
