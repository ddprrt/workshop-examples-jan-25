pub trait Print {
    fn print(&self);
}

impl<T: std::fmt::Display> Print for T {
    fn print(&self) {
        println!("{}", self);
    }
}
