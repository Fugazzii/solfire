#[macro_export]
macro_rules! env_var {
    ($key: expr) => {
        dotenvy::var($key)
            .expect(
                format!("ENV VARIABLE MISSING: {} is not defined", $key).as_str()
            )
            .as_str()
    }
}
