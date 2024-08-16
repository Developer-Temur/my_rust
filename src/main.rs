use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("kattasi {}", self.x);
        }else {
            println!("kattasi {}", self.y)
        }
    }
}

fn main() {
    let numbers = Pair::new(5, 9);
    numbers.cmp_display();
}//241