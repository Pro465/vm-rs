use crate::bits::Bits;
use crate::Vm;
use crate::Key;

use std::io::Write;
use std::collections::HashMap;
use std::ops::{ Add, Sub, Mul, Div, Rem, Shr, Shl, BitAnd, BitOr, BitXor};
use std::convert::{ From, TryInto };
use std::cmp::Ord;
use std::fmt::{ Debug, Display };

type Func = fn(&mut Vm);

pub static mut BUILTINS: Option<HashMap<Key,Func>> = None;

macro_rules! insert {
    ($res: ident, $($name: literal => $val: expr),*) => {
        $(
            $res.insert($name.to_vec(), $val);
        )*
    }
}

macro_rules! insert_op {
    ($res: ident, 
        $(
            $tys: ident => ($($mods: ident),*)
        ),*
    ) => {
        $(
            $(
               $res.insert(
                   concat!(
                       stringify!($mods),
                       "_",
                       stringify!($tys)
                   ).as_bytes().to_vec(), 
                   $mods::<$tys>);
            )*
        )*
    }
}

pub fn init() -> HashMap<Key,fn(&mut Vm)> {
    let mut res: HashMap<Key,fn(&mut Vm)> = HashMap::new();

    insert! (
        res,

        b"is_def" => is_def,
        b"print" => print_str
    );

    insert_op!(
        res,

        u16 => (add, sub, mul, div, modulo, 
               bit_and, bit_or, bit_xor, 
               le, ge, lt, gt, eq, print),
        u32 => (add, sub, mul, div, modulo, 
               bit_and, bit_or, bit_xor, 
               le, ge, lt, gt, eq, print),
        u64 => (add, sub, mul, div, modulo, 
               bit_and, bit_or, bit_xor, 
               le, ge, lt, gt, eq, print),
        u128 => (add, sub, mul, div, modulo, 
               bit_and, bit_or, bit_xor, 
               le, ge, lt, gt, eq, print),
        i16 => (add, sub, mul, div, modulo, 
               bit_and, bit_or, bit_xor, 
               le, ge, lt, gt, eq, print),
        i32 => (add, sub, mul, div, modulo, 
               bit_and, bit_or, bit_xor, 
               le, ge, lt, gt, eq, print),
        i64 => (add, sub, mul, div, modulo, 
               bit_and, bit_or, bit_xor, 
               le, ge, lt, gt, eq, print),
        i128 => (add, sub, mul, div, modulo, 
               bit_and, bit_or, bit_xor, 
               le, ge, lt, gt, eq, print)
    );
    
    res
}

fn print<T>(vm: &mut Vm)
    where T: Display + From<u8> + Shl<Output=T>

         + BitAnd<Output=T> + BitOr<Output=T> + Copy + Bits
 
         + Sub<Output=T> {

    println!("{}", get::<T>(vm));
}

fn print_str(vm: &mut Vm) {
    while let Some(x) = vm.stack().pop() {
        print!("{}", x as char);
    }

    std::io::stdout().flush().unwrap();
}


fn is_def(vm_state: &mut Vm) {
    let name_num = vm_state.get_u32();

    let mut name = Vec::new();

    for _ in 0..name_num {
        vm_state.incr();
        name.push(vm_state.curr_byte())
    }

    let is_def = vm_state.top_frame().contains_key(&name);

    vm_state.stack().push(is_def as u8)
}

fn get<T> (vm: &mut Vm) -> T
where T: From<u8> + Shl<Output=T>

         + BitAnd<Output=T> + BitOr<Output=T> + Copy + Bits
 
         + Sub<Output=T> {
     let eight = T::from(8);

     let mut res = T::from(vm.top()) << T::from(T::BITS) - eight;

     for i in 1..T::BITS/8 {
         res = res | (T::from(vm.top()) 
             << T::from(T::BITS) 
             - T::from(i*8) 
             - eight);
     }

     res
}

macro_rules! op {
    (
        $($i: ident $(: ($($t: tt)*))? => $op: tt),* 
    ) => {
        $(fn $i<T>(vm: &mut Vm) 
           where T: From<u8> + TryInto<u8> + Shr<Output=T> + Shl<Output=T> 

                 + BitAnd<Output=T> + BitOr<Output=T> + Copy + Bits
 
                 + Sub<Output=T> $(+ $($t)*)?,

                <T as TryInto<u8>>::Error: Debug {
            let a = get::<T>(vm);
            let b = get::<T>(vm);

            let res = b $op a;

            for i in 1..T::BITS/8 {
                vm.stack().push(((res >> T::from(i*8)) & T::from(0xff)).try_into().unwrap());
            };
         })*
    }
}

op!(
    add: (Add<Output=T>) => +,
    mul: (Mul<Output=T>) => *,
    div: (Div<Output=T>) => /,
    modulo: (Rem<Output=T>) => %,
    bit_xor: (BitXor<Output=T>) => ^,

    sub => -,
    bit_and => &,
    bit_or => |
);

macro_rules! op_bool {
    (
        $($i: ident $(: ($($t: tt)*))? => $op: tt),* 
    ) => {
        $(fn $i<T>(vm: &mut Vm) 
           where T: From<u8> + Shl<Output=T>

                 + BitAnd<Output=T> + BitOr<Output=T> + Copy + Bits
 
                 + Sub<Output=T> $(+ $($t)*)? {
            let a = get::<T>(vm);
            let b = get::<T>(vm);

            let res = b $op a;

            vm.stack().push(res.into());
         })*
    }
}

op_bool! (
    gt: (Ord) => >,
    lt: (Ord) => <,
    ge: (Ord) => >=,
    le: (Ord) => <=,
    eq: (Ord) => ==
);
