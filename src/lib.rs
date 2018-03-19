#[macro_export]
macro_rules! dump {
    ($ex: expr) => (
        println!("{} => {:?}", stringify!($ex), $ex);
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn kick_the_tyres() {
        let s = String::from("hi");
        dump!(s);
    }
}