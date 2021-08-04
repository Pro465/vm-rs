use crate::exec;
use crate::Frame;
use crate::ByteCode;

use std::convert::TryInto;

pub struct Vm {
    pub finished: bool,
    pub code: Vec<u8>,
    pub pc: u32,
    pub frames: Vec<(Frame,u32)>,
    pub globals: Frame,
    pub stack: Vec<Vec<u8>>,
    pub mem: [u8; 256]
}

impl Vm {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            finished: false,
            code,
            pc: 0,
            frames: Vec::new(),
            globals: Frame::new(),
            stack: vec![ Vec::new() ],
            mem: [0; 256],
        }
    }

    pub fn run(&mut self) {
        while !self.finished && self.pc() < self.len_of_code() {
            self.eval();
        }
    }

    #[inline]
    fn eval(&mut self) {
        let code: ByteCode = self.curr_byte().try_into().unwrap();
        self.pc = exec(self,code);
    }

    #[inline]
    pub fn top_frame(&mut self) -> &mut Frame {
        let len = self.frames.len();

        if len == 0 {
            &mut self.globals
        }
        else {
            &mut self.frames[len - 1].0
        }
    }

    #[inline]
    pub fn get_u32(&mut self) -> u32 {
        let mut res = 0;

        for i in 0..4 {
	    self.incr();
	    res |= (self.curr_byte() as u32) << (i*8)
        }
	    
	res
    }
    #[inline]
    pub fn top(&mut self) -> u8 {
        self.stack().pop().unwrap()
    }
    #[inline]
    pub fn stack(&mut self) -> &mut Vec<u8> {
        let idx = self.stack.len() - 1;
        &mut self.stack[idx]
    }
    #[inline]
    pub fn curr_byte(&self) -> u8 {
        self.code[self.pc()]
    }
    #[inline]
    pub fn pc(&self) -> usize {
        self.pc as usize
    }
    #[inline]
    pub fn len_of_code(&self) -> usize {
        self.code.len()
    }
    #[inline]
    pub fn incr(&mut self) {
        self.pc += 1
    }
}
