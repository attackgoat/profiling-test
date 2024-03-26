#[doc(hidden)]
#[macro_export]
macro_rules! tracy_span {
    ($name:expr, $callstack_depth:expr) => {
        ()
    };
}
