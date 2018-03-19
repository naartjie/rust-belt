#[macro_use]

mod dev {

    #[macro_export]
    macro_rules! dump {
        ($ex: expr) => (
            println!("{} => {:?}", stringify!($ex), $ex);
        )
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn kick_the_tyres() {
        let s = "hi";
        dump!(s);
    }
}