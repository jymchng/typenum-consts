macro_rules! debug_eprintln {
    ($($arg: tt)*) => {

        #[cfg(debug_assertions)]
        #[cfg(feature = "debug")]
        ::std::eprintln!($($arg)*)
    };
}

pub(crate) use debug_eprintln;
