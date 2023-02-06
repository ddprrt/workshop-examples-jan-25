use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread,
};

use rand::{prelude::IteratorRandom, thread_rng};

#[derive(Default, Debug)]
struct Lotto {
    numbers: Vec<usize>,
}

impl Lotto {
    pub fn new(amount: usize, max: usize) -> Self {
        let pot = 1..=max;
        let mut rng = thread_rng();
        Self {
            numbers: pot.choose_multiple(&mut rng, amount),
        }
    }
}

trait Unlockable {
    fn unlock(self);
}

impl<'a, T> Unlockable for MutexGuard<'a, T> {
    fn unlock(self) {}
}

fn main() {
    let result = Vec::new();
    let result = Mutex::new(result);
    let result = Arc::new(result);
    let pairs = [(6, 45), (5, 50), (6, 49)];
    let mut handles = Vec::new();

    for (take, from) in pairs {
        let result = result.clone();
        let handle = thread::spawn(move || {
            let lotto = Lotto::new(take, from);
            {
                let mut guard = result.lock().unwrap();
                guard.push(lotto);
                guard.unlock();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = result.lock().unwrap();
    for lotto in result.iter() {
        println!("{:?}", lotto.numbers);
    }
}
