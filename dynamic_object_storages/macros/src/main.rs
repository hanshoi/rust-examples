extern crate serde;

use serde::{Deserialize, Serialize};

macro_rules! register {
    ($($i:expr => $t:expr),*) => {{
        fn handle_a(x: &str) {
            match x {
                $($i => println!("{}", $t.foo($t.get(""))),)*
                _ => println!("default"),
            }
        };
        handle_a
    }};
}


trait A<T> {
    fn get(&self, raw: &str) -> T;
    fn foo(&self, input: T) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
struct Bob{
    name: String
}

struct B {
}

impl A<Bob> for B {

    fn get(&self, _: &str) -> Bob {
        Bob{name: String::from("Builder")}
    }

    fn foo(&self, b: Bob) -> String {
        b.name
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct Curtis {
    job: String
}

struct C {
}

impl A<Curtis> for C {
    fn get(&self, _: &str) -> Curtis {
        Curtis{job: String::from("Curtainer")}
    }

    fn foo(&self, b: Curtis) -> String {
        b.job
    }
}


fn main() {
    let handler = register!(
        "bob" => B{},
        "curtis" => C{}
    );
    handler("b");
    handler("bob");
    handler("curtis");
}
