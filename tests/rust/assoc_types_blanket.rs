pub trait ReprC {
    type CType;
}

#[repr(C)]
pub struct Wrapper<T>(pub T);

#[repr(C)]
pub struct Pair<T, U>(pub T, pub U);

impl<T> ReprC for Wrapper<T> {
    type CType = T;
}

impl<T> ReprC for Pair<T, T> {
    type CType = Wrapper<T>;
}

pub trait Outer {
    type CType;
}

impl<T> Outer for Wrapper<T> {
    type CType = <Wrapper<T> as ReprC>::CType;
}

#[no_mangle]
pub extern "C" fn blanket_identity(
    value: <Wrapper<u32> as ReprC>::CType,
) -> <Wrapper<u32> as ReprC>::CType {
    value
}

#[no_mangle]
pub extern "C" fn blanket_nested(
    value: <Wrapper<u16> as Outer>::CType,
) -> <Wrapper<u16> as Outer>::CType {
    value
}

#[no_mangle]
pub extern "C" fn blanket_repeated(
    value: <Pair<u8, u8> as ReprC>::CType,
) -> <Pair<u8, u8> as ReprC>::CType {
    value
}

const _: () = {
    #[unsafe(export_name = "blanket_nested_const_export")]
    unsafe extern "C" fn hidden(value: <Wrapper<u32> as ReprC>::CType) -> u32 {
        value
    }
};
