macro_rules! debug_eprintln {
    ($($arg: tt)*) => {

        #[cfg(debug_assertions)]
        #[cfg(__debug_tnconst)]
        ::std::eprintln!($($arg)*)
    };
}

pub(crate) use debug_eprintln;
