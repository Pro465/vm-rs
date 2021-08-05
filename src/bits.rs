pub trait Bits {
    const BITS: u8;
}

macro_rules! impl_trait {
    ($($i: ident => $num: literal),*) => {
        $(
            impl Bits for $i {
                const BITS: u8 = $num;
            }
        )*
    }
}

impl_trait! (
    u8 => 8,
    u16 => 16,
    u32 => 32,
    u64 => 64,
    u128 => 128,
    i8 => 8,
    i16 => 16,
    i32 => 32,
    i64 => 64,
    i128 => 128
);
