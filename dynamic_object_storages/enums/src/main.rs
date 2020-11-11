extern crate serde;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

trait A {
    fn get(&self, raw: &str) -> Content;
    fn foo(&self, input: Content) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
struct Bob{
    name: String
}

struct B {
}

impl A for B {

    fn get(&self, _: &str) -> Content {
        Content::BOB(Bob{name: String::from("Builder")})
    }

    fn foo(&self, b: Content) -> String {
        match b {
            Content::BOB(bob) => bob.name,
            _ => String::from("")
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct Curtis {
    job: String
}

struct C {
}

impl A for C {
    fn get(&self, _: &str) -> Content {
        Content::CURTIS(Curtis{job: String::from("Curtainer")})
    }

    fn foo(&self, b: Content) -> String {
        match b {
            Content::CURTIS(curtis) => curtis.job,
            _ => String::from("")
        }
    }
}

enum Content {
    CURTIS(Curtis),
    BOB(Bob),
}


fn main() {
    let mut map: HashMap<&str, Box<dyn A>> = HashMap::new();
    map.insert("b", Box::new(B{}));
    map.insert("c", Box::new(C{}));

    for (_, val) in map.iter() {
        println!("{}", val.foo(val.get("")));
    }
}
