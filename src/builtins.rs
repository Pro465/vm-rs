use crate::Vm;
use crate::Key;

use std::io::Write;
use std::collections::HashMap;

type Func = fn(&mut Vm);

pub static mut BUILTINS: Option<HashMap<Key,Func>> = None;

pub fn init() -> HashMap<Key,fn(&mut Vm)> {
    let mut res: HashMap<Key,fn(&mut Vm)> = HashMap::new();

    res.insert(b"print".to_vec(),print);
    res.insert(b"is_def".to_vec(),is_def);

    res
}

fn print(vm: &mut Vm) {
    while let Some(top) = vm.stack().pop() {
        print!("{}",top as char);
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
