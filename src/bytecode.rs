use std::convert::TryFrom;

macro_rules! impl_tryfrom {
    ($priv: vis enum $i: ident<$t: ident> { 
        $($v: ident = $l: literal),* 
    }) => {
        #[repr($t)]
        $priv enum $i {
            $($v = $l),*
        }

        impl TryFrom<$t> for $i {
            type Error = ();
            fn try_from(t: $t) -> Result<Self,()> {
                $( if t == $l { Ok(Self::$v) } else )*
                { Err(()) }
            }
        }
    }
}

impl_tryfrom!(

pub enum ByteCode<u8> {
    Exit = 0x00,
    Jmp = 0x01,
    Jif = 0x02,
    Call = 0x03,
    Ret = 0x04,
    Push = 0x05,
    Putc = 0x06,
    Puts = 0x07,
    Def = 0x08,
    Set = 0x09,
    Get = 0x0A,
    Load = 0x0B,
    Store = 0x0C,
    Swap = 0x0D,
    Rev = 0x0E,
    Pop = 0x0F,
    Label = 0x10,

    Add = 0x11,
    Sub = 0x12,
    Mul = 0x13,
    Div = 0x14,
    Mod = 0x15,

    ShR = 0x16,
    ShL = 0x17,
    RotR = 0x18,
    RotL = 0x19,
    And = 0x1A,
    Or = 0x1B,
    Not = 0x1C,

    Eq = 0x1D,
    Gt = 0x1E,
    Lt = 0x1F,
    Ge = 0x20,
    Le = 0x21,

    Len = 0x22,
    Dup = 0x23
}

);
