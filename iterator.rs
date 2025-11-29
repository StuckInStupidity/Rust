struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    // The return type is `Option<T>`:
    // * When the `Iterator` is finished, `None` is returned.
    // * Otherwise, the next value is wrapped in `Some` and returned.
    // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
    // will never return `None`, and `Some` is always returned.

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {

    let mut sequence = 0..3;
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }

    /*Four consecutive `next` calls on 0..3
    > Some(0)
    > Some(1)
    > Some(2)
    > None

    Iterate through 0..3 using `for`
    > 0
    > 1
    > 2

    The first four terms of the Fibonacci sequence are:
    > 0
    > 1
    > 1
    > 2

    The next four terms of the Fibonacci sequence are:
    > 3
    > 5
    > 8
    > 13

    Iterate the following array [1, 3, 3, 7]
    > 1
    > 3
    > 3
    > 7*/
}