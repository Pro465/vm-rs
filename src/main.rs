use std::env::{var,args};

use vm_rs::Vm;
use vm_rs::{BUILTINS,init};

fn main() {
	let path;

	if let Some(mut x) = args().nth(1) {
		if x.chars().next().unwrap()
			.is_alphabetic() || x.starts_with("./") {
			if x.starts_with("./") {
				x = x[2..].to_string(); 
			}

			path = var("PWD").unwrap_or_default() + "/" + &x;
		}
		else {
			path = x;
		}
	}
	else { help(); }

	let contents = std::fs::read(path).expect("file not found");	
	
        unsafe {
            BUILTINS = Some(init());
        }

	let mut vm = Vm::new(contents);

	vm.run();
}

fn help() -> ! {
	println!(r#"USAGE: 
   vm <filename>
"#	);
	std::process::exit(101);
}
