struct Fibonacci {
    curr: u128,
    next: u128,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {
    let iter = Fibonacci::default()
        .into_iter()
        .skip(10)
        .take(15)
        .enumerate();

    for x in iter {
        println!("{:>3} {:>10}", x.0, x.1);
    }

    let sum: u128 = Fibonacci::default().into_iter().take(6).sum();
    println!("{sum}");

    for x in Fibonacci::default().enumerate() {
        println!("{:>3} {:>10}", x.0, x.1);
    }
}
