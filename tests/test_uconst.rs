use core::marker::PhantomData;
use typenum::{UInt, UTerm, Unsigned, B0, B1, U69};
use typenum_consts::uconst;

struct Wrapper<T: Unsigned>(PhantomData<T>);

impl<T> Wrapper<T>
where
    T: Unsigned,
{
    fn give_t_as_usize(&self) -> usize {
        <T as Unsigned>::USIZE
    }
}

#[test]
fn test_uconst_one() {
    let wrapper = Wrapper::<uconst![84938493]>(PhantomData);

    assert_eq!(wrapper.give_t_as_usize(), <uconst![84938493]>::USIZE);
}
