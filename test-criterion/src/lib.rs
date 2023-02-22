pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { curr: 0, next: 1 }
    }
}

pub fn fibonacci_iter(n: usize) -> u64 {
    Fibonacci::default().nth(n).unwrap()
}

pub fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

pub fn fibonacci_rec(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}
