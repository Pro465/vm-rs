use crate::Vm;
use crate::ByteCode;
use crate::Frame;
use crate::BUILTINS;

use std::io::Write;
use std::convert::TryInto;

pub fn exec(vm_state: &mut Vm,code: ByteCode) -> u32 {
    match code {
        ByteCode::Exit => vm_state.finished = true,
        
        ByteCode::Putc => {
            print!("{}",vm_state.top() as char);
            std::io::stdout().flush().unwrap();
        }
        ByteCode::Puts => {
            print!("{}",vm_state.top());
            std::io::stdout().flush().unwrap();
        }
        ByteCode::Push => {
            vm_state.incr();

            let num = vm_state.curr_byte();

            for _ in 0..num {
                vm_state.incr();

                let data = vm_state.curr_byte();
                vm_state.stack().push(data);
            }
        }
        ByteCode::Jmp => return vm_state.get_u32(),
        ByteCode::Jif => {
            let addr = vm_state.get_u32();
            
            if vm_state.top() > 0 {
                return addr;
            }
        }
        ByteCode::Pop => {
            vm_state.top();
        }
        ByteCode::Call => return call(vm_state),
        ByteCode::Ret => return ret(vm_state),
        ByteCode::Def => def(vm_state),
        ByteCode::Get => get(vm_state),
        ByteCode::Set => set(vm_state),
        ByteCode::Load => load(vm_state),
        ByteCode::Store => store(vm_state),

        ByteCode::Add => add(vm_state),
        ByteCode::Sub => sub(vm_state),
        ByteCode::Mul => mul(vm_state),
        ByteCode::Div => div(vm_state),
        ByteCode::Mod => modulo(vm_state),

        ByteCode::RotR => rot_r(vm_state),
        ByteCode::RotL => rot_l(vm_state),
        ByteCode::ShR => sh_r(vm_state),
        ByteCode::ShL => sh_l(vm_state),
        ByteCode::Not => not(vm_state),
        ByteCode::And => and(vm_state),
        ByteCode::Or => or(vm_state),

        ByteCode::Eq => eq(vm_state),
        ByteCode::Gt => gt(vm_state),
        ByteCode::Lt => lt(vm_state),
        ByteCode::Ge => ge(vm_state),
        ByteCode::Le => le(vm_state),

        ByteCode::Swap => swap(vm_state),
        ByteCode::Rev => vm_state.stack().reverse(),

        ByteCode::Label => label(vm_state),
        ByteCode::Len => len(vm_state),
        ByteCode::Dup => dup(vm_state),
    }

    vm_state.pc + 1
}

fn get_name(vm_state: &mut Vm) -> Vec<u8> {
    let name_num = vm_state.get_u32();
    
    let mut name = Vec::new();

    for _ in 0..name_num {
        vm_state.incr();
        name.push(vm_state.curr_byte())
    }

    name
}

fn call_common(vm_state: &mut Vm) {
    let num = vm_state.get_u32();
    let len = vm_state.stack().len();

    let args = vm_state.stack().split_off(len - num as usize);
    
    vm_state.stack.push(args);
    vm_state.frames.push((Frame::new(),vm_state.pc));
}

fn call(vm_state: &mut Vm) -> u32 {
    vm_state.incr();
    let is_name = vm_state.curr_byte();

    if is_name == 0 {
        let addr = vm_state.get_u32();

        call_common(vm_state);

        addr
    }
    else {
        let name = get_name(vm_state);
        if let Some(x) = vm_state.top_frame().get(&name) {
            let mut addr = 0;

            for i in 0..4 {
                 addr |= (x[x.len() - i - 1] as u32) << (i*8);
            }
 
            call_common(vm_state);

            addr
        }
        else if let Some(x) = vm_state.globals.get(&name) {
            let mut addr = 0;

            for i in 0..4 {
                 addr |= (x[x.len() - i - 1] as u32) << (i*8);
            }

            call_common(vm_state);

            addr
        }
        else {
            call_common(vm_state);
            
            unsafe {
                BUILTINS.as_ref().unwrap()[&name](vm_state);
            }

            ret(vm_state);

            vm_state.pc + 1
        }
    }
}

fn ret(vm_state: &mut Vm) -> u32 {
    let (_,r_addr) = vm_state.frames.pop().unwrap();

    let mut returned = vm_state.stack.pop().unwrap();

    vm_state.stack().append(&mut returned);

    r_addr
}

fn len(vm_state: &mut Vm) {
    let len: u32 = vm_state.stack.len().try_into().unwrap();

    for i in 0..4 {
        vm_state.stack().push((len >> (24 - i*8)) as u8);
    }
}

fn def(vm_state: &mut Vm) {
    let val = vm_state.stack.pop().unwrap();
    let name = get_name(vm_state);

    vm_state.top_frame().insert(name,val).unwrap();
}

fn label(vm_state: &mut Vm) {
    let name = get_name(vm_state);
    
    let val = vm_state.pc;

    vm_state.globals.insert(name,vec![
         (val >> 24) as u8,
         (val >> 16) as u8,
          (val >> 8) as u8,
                 val as u8]);
}


fn set(vm_state: &mut Vm) {
    let name = get_name(vm_state);
    let val = vm_state.stack.pop().unwrap();

    if let Some(x) = vm_state.top_frame().get_mut(&name) {
        *x = val;
    }
    else {
        assert!(matches!(vm_state.globals.insert(name,val),Some(_)))
    }
}

fn get(vm_state: &mut Vm) {
    let name = get_name(vm_state);
    let mut val;

    if let Some(x) = vm_state.top_frame().get(&name) {
        val = x.clone()
    }
    else {
        val = vm_state.globals.get(&name).unwrap().clone();
    }

    vm_state.stack().append(&mut val);
}

fn load(vm_state: &mut Vm) {
    vm_state.incr();

    let addr = vm_state.curr_byte();

    let val = vm_state.mem[addr as usize];

    vm_state.stack().push(val);
}

fn store(vm_state: &mut Vm) {
    vm_state.incr();

    let addr = vm_state.curr_byte();

    let prev = vm_state.mem[addr as usize];

    vm_state.mem[addr as usize] = vm_state.top();

    vm_state.stack().push(prev);
}

fn rot_r(vm_state: &mut Vm) {
    let a = vm_state.top();
    let b = vm_state.top();

    vm_state.stack().push(b.rotate_right(a.into()))
}

fn rot_l(vm_state: &mut Vm) {
    let a = vm_state.top();
    let b = vm_state.top();

    vm_state.stack().push(b.rotate_left(a.into()))
}

fn not(vm_state: &mut Vm) {
    let a = vm_state.top();

    vm_state.stack().push(!a)
}

fn swap(vm_state: &mut Vm) {
    let a = vm_state.top();
    let b = vm_state.top();

    vm_state.stack().push(a);
    vm_state.stack().push(b);
}


fn dup(vm_state: &mut Vm) {
    let a = vm_state.top();

    vm_state.stack().push(a);
    vm_state.stack().push(a);
}

macro_rules! op {
    ($($t: ident => $op: tt),*) => {
        $(
        #[inline]
        fn $t(vm_state: &mut Vm) {
            let a = vm_state.top();
            let b = vm_state.top();
            let res = (b $op a) as u8;

            vm_state.stack().push(res);
        }
        )*
    }
}

//    {
//       u8 => {0},
//       u16 => { 0, 8 },
//       u32 => { 0, 8 16, 24 },
//       u64 { 0, 8, 16, 32, 40, 48, 56 }
//    }

op!(
    add => +, 
    sub => -,
    mul => *,
    div => /,
    modulo => %,
    
    sh_r => >>,
    sh_l => <<,
    and => &,
    or => |,

    eq => ==,
    gt => >,
    lt => <,
    ge => >=,
    le => <=
);
