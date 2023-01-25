use rand::{rngs::ThreadRng, seq::SliceRandom};

struct Point {
    x: i32,
    y: i32,
}

struct Lotto<'a> {
    take: usize,
    pot: Vec<usize>,
    rng: &'a mut ThreadRng,
}

impl<'a> Lotto<'a> {
    // constructor function
    fn new(take: usize, from: usize, rng: &'a mut ThreadRng) -> Self {
        Self {
            take,
            pot: (1..=from).collect(),
            rng,
        }
    }

    fn shuffle(&mut self) -> &mut Self {
        self.pot.shuffle(self.rng);
        self
    }

    fn take(&self) -> Vec<usize> {
        self.pot
            .iter()
            .take(self.take)
            .map(ToOwned::to_owned)
            .collect()
    }
}

fn main() {
    let p = Point { x: 10, y: 10 };

    println!("{} {}", p.x, p.y);

    let mut rng = rand::thread_rng();
    let mut lotto = Lotto::new(5, 50, &mut rng);
    let result = lotto.shuffle();
    println!("{:?}", result.take());

    let mut lotto2 = Lotto::new(2, 12, &mut rng);
    let result = lotto2.shuffle();
    println!("{:?}", result.take());

    drop(lotto2);

    let mut vec = vec![1, 2, 3, 4];

    for i in vec.iter_mut() {
        *i = 10;
    }

    println!("{:?}", vec);
}
