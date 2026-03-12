mod animal {
    pub mod dog {
        pub fn say_hi() {
            println!("inu!!");
        }
    }
    mod cat {
        fn say_hi() {
            println!("neko!!");
        }
    }
}

pub mod people;

use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    crate::animal::dog::say_hi();
    animal::dog::say_hi();

    people::say_hi();

    module_test::lib_people::say_hi();

    use animal::dog;
    dog::say_hi();

    // Animal::Cat::say_hi();
}

fn func1() -> Result {
    Result::Ok(())
}
fn func2() -> IoResult<()> {
    IoResult::Ok(())
}

// fn test() {
//     dog::say_hi();
// }
