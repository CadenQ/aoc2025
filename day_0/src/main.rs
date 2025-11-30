struct Number {
    odd: bool,
    value: i32
}

impl Number {
    fn is_positive(self) -> bool {
        self.value > 0
    }
}

struct Pair<T> {
    a: T,
    b: T,
}

fn main() {
    println!("Hello, world!");
    let a: Number = Number { odd: false, value: 2 };
        
    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: 1, b: false };
    
}
