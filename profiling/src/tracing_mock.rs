pub enum Level {
    Info,
}

pub struct Span;

impl Span {
    #[inline(always)]
    pub fn enter(&self) {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! tracing_span {
    ($level:expr, $name:expr) => {
        $crate::tracing::Span
    };
}
