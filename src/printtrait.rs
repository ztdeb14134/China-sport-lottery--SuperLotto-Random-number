pub trait PrintResult {
    fn printout(self);
}
impl PrintResult for Result<Vec<String>, ()> {
    fn printout(self) {
        self.unwrap().into_iter().for_each(|s| println!("{}", s));
    }
}
impl PrintResult for [&str; 3] {
    fn printout(self) {
        self.iter().for_each(|s| println!("{}", s));
    }
}
