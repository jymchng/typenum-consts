macro_rules! debug_eprintln {
    ($($arg: tt)*) => {

        #[cfg(debug_assertions)]
        #[cfg(feature = "debug")]
        ::std::eprintln!($($arg)*)
    };
}

pub(crate) use debug_eprintln;

macro_rules! no_std_format {
    ($(&mut)? $buff:ident, $($arg:tt)*) => {
        $crate::no_std_formatter::show(&mut $buff, format_args!($($arg)*))
    };
}

pub(crate) use no_std_format;
