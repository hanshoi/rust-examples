fn buzz() {
    println!("buzz");
}

fn fizz() {
    println!("fizz");
}

fn fizzbuzz() {
    println!("fizzbuzz");
}

fn main() {
    for i in 0 .. 100 {
        let fb = (i%3 == 0, i%5 == 0);
        match fb {
            (true, true) => fizzbuzz(),
            (true, false) => fizz(),
            (false, true) => buzz(),
            _ => println!("{}", i),
        }
    }
}
