#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Project {
    price: f64,
}

#[derive(Debug)]
struct MaintenanceHours {
    hours: f64,
    rate: f64,
}

impl PartialEq for MaintenanceHours {
    fn eq(&self, other: &Self) -> bool {
        self.hours * self.rate == other.hours * other.rate
    }
}

impl PartialOrd for MaintenanceHours {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let bill = self.hours * self.rate;
        let other_bill = other.hours * other.rate;
        bill.partial_cmp(&other_bill)
    }
}

trait Billable: core::fmt::Debug {
    fn bill(&self) -> f64 {
        0.0
    }
}

impl Billable for Project {
    fn bill(&self) -> f64 {
        self.price
    }
}

impl Billable for MaintenanceHours {
    fn bill(&self) -> f64 {
        self.hours * self.rate
    }
}

impl Billable for f64 {
    fn bill(&self) -> f64 {
        *self
    }
}

fn print_bill<T: Billable + ?Sized>(billable: &T) {
    println!("{}", billable.bill());
}

fn print_bills<T, U>(a: &T, b: &U)
where
    T: Billable + core::fmt::Debug,
    U: Billable,
{
    println!("{:?} {:?}", a, b);
    println!("{} {}", a.bill(), b.bill());
}

#[derive(Debug)]
enum Billables {
    Project(Project),
    MaintenanceHours(MaintenanceHours),
}

impl Billable for Billables {
    fn bill(&self) -> f64 {
        match self {
            Billables::Project(project) => project.bill(),
            Billables::MaintenanceHours(hours) => hours.bill(),
        }
    }
}

fn main() {
    let p = Project { price: 5000.0 };
    let h = MaintenanceHours {
        hours: 50.0,
        rate: 100.0,
    };
    print_bills(&p, &h);

    println!("{:?}", p);

    let q = Project { price: 6000.0 };
    println!("{}", p < q);

    let i = MaintenanceHours {
        hours: 100.0,
        rate: 50.0,
    };

    print_bill(&p);

    let vec: Vec<Box<dyn Billable>> = vec![Box::new(p), Box::new(q), Box::new(i)];
    for element in vec {
        print_bill(element.as_ref());
    }

    let arr = [
        Billables::Project(Project { price: 1000.0 }),
        Billables::Project(Project { price: 2000.0 }),
        Billables::Project(Project { price: 3000.0 }),
        Billables::MaintenanceHours(MaintenanceHours {
            rate: 100.0,
            hours: 100.0,
        }),
    ];

    for elem in arr {
        print_bill(&elem);
    }
}
