
macro_rules! cppmax {
    ($x:expr, $y:expr) => {
        if $x + $y < 5 {
            $y
        } else {
            $x
        }
    };
}
