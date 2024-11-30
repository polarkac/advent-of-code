#[macro_export]
macro_rules! include_input {
    ($year:literal / $day:literal) => {
        include_str!(
            concat!(
                "../../input/",
                stringify!($year),
                "/",
                stringify!($day),
                ".txt"
            )
        )
    };
}
