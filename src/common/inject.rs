#[macro_export]
macro_rules! inject {
    ($key: expr) => {
        Data::new($key)
    }
}
