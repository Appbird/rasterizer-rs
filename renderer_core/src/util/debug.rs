#[macro_export]
macro_rules! snapshot {
    ($t:expr) => {
        eprintln!(
            "[snapshot] {} = {:?} at {}:{}",
            stringify!($t),
            $t,
            file!(),
            line!()
        );
    };
}
#[macro_export]
macro_rules! assert_cond {
    ($t1:expr, $t2:expr) => {
        if ($t1 != $t2) {
            eprintln!(
                "[[assertion failed]] {} == {} at {}:{}",
                stringify!($t1),
                stringify!($t2),
                file!(),
                line!()
            );
            eprintln!("where {} = {:?}, {} = {:?}", stringify!($t1), $t1, stringify!($t2), $t2);
            panic!();
        }
    };
}