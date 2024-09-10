pub mod controls;
pub mod devices;
pub mod error;
pub mod signals;
pub mod spn;

type Status<T> = Result<T, error::StatusCode>;

pub(crate) mod __sealed {
    #[macro_export]
    #[allow(clippy::crate_in_macro_def)]
    macro_rules! seal {
        ($($t:ty),*) => {
            $(
                impl crate::__sealed::Sealed for $t {}
            )*
        };
    }
    pub trait Sealed {}
}

const DEFAULT_TIMEOUT: f64 = 0.05;
