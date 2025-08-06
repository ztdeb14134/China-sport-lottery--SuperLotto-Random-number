use crate::sql::func::insert_number;

pub trait PrintResult {
    fn printout(self);
}
impl PrintResult for Result<Vec<String>, ()> {
    fn printout(self) {
        insert_number(self.clone().unwrap());
        self.unwrap().into_iter().for_each(|s| println!("{}", s));
    }
}
impl<const N: usize> PrintResult for [&str; N] {
    fn printout(self) {
        insert_number(self.iter().map(|s| s.to_string()).collect());
        self.iter().for_each(|s| println!("{}", s));
    }
}
