/// Provides utilities for writing formatted data into a byte buffer.
///
/// The `WriteTo` struct allows writing formatted data into a byte buffer. It implements
/// the `fmt::Write` trait, enabling it to be used with the formatting macros from the
/// `core::fmt` module.
///
/// The `show` function is a convenience function that accepts a mutable reference to a byte
/// buffer and a `fmt::Arguments` object representing the formatted data. It internally
/// creates a `WriteTo` instance and writes the formatted data into the buffer. If the buffer
/// is large enough to hold the formatted data, the function returns a reference to the
/// string slice containing the formatted data. Otherwise, it returns an error of type
/// `fmt::Error`.
///
/// # Safety
///
/// The `WriteTo` struct uses unsafe code internally to create a string slice from a byte
/// slice without performing UTF-8 validation. It relies on the caller to ensure that the
/// byte buffer contains valid UTF-8 data after writing formatted data into it. If the
/// contents of the buffer are not valid UTF-8, using the `as_str` method may lead to
/// undefined behavior.
///
/// # Examples
///
/// ```skip
/// let mut buf = [0; 10];
/// let result = no_std_format::show(&mut buf, format_args!("Hello, {}!", "world"));
/// assert_eq!(result, Ok("Hello, world!"));
/// ```
use core::cmp::min;
use core::fmt;
use core::str::from_utf8_unchecked;

pub struct WriteTo<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl<'a> WriteTo<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        WriteTo { buf, len: 0 }
    }

    pub unsafe fn as_str(self) -> Option<&'a str> {
        if self.len <= self.buf.len() {
            Some(unsafe { from_utf8_unchecked(&self.buf[..self.len]) })
        } else {
            None
        }
    }
}

impl<'a> fmt::Write for WriteTo<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.len > self.buf.len() {
            return Err(fmt::Error);
        }

        let rem = &mut self.buf[self.len..];
        let raw_s = s.as_bytes();
        let num = min(raw_s.len(), rem.len());

        rem[..num].copy_from_slice(&raw_s[..num]);
        self.len += raw_s.len();

        if num < raw_s.len() {
            Err(fmt::Error)
        } else {
            Ok(())
        }
    }
}

pub fn show<'a>(buf: &'a mut [u8], arg: fmt::Arguments) -> Result<&'a str, fmt::Error> {
    let mut w = WriteTo::new(buf);
    fmt::write(&mut w, arg)?;
    // SAFETY: In this crate, only only digits and ASCII characters are used for formatting.
    unsafe { w.as_str() }.ok_or(fmt::Error)
}
