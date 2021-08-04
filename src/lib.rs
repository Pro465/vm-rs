mod vm;
pub use vm::Vm;

mod bytecode;
pub use bytecode::ByteCode;

mod exec;
pub use exec::exec;

mod builtins;
pub use builtins::{BUILTINS,init};

pub type Key = Vec<u8>;

pub type Frame = std::collections::HashMap<Key,Vec<u8>>;
