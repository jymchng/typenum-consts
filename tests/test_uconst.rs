use core::marker::PhantomData;
use typenum::Unsigned;
use typenum_consts::{tnconst, uconst};

struct Wrapper<T: Unsigned>(PhantomData<T>);

#[test]
fn test_uconst_one() {
    let _wrapper = Wrapper::<uconst![84938493]>(PhantomData);

    assert_eq!(84938493_usize, <uconst![84938493]>::USIZE);
}
