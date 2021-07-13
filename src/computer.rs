use core::num::*;
use std::vec::Vec;

use arrayfire::af_print;
use arrayfire::Array;
use arrayfire::c32;
use num::Complex;

use crate::registers::*;

pub struct QCSim {
    size: u64,
    qreg: QRegister,
    creg: CRegister,
}

impl QCSim {
    //Creates a new Quantum Computer Simulator Object
    pub fn new(reg_size : u64) -> QCSim {
        QCSim {
            size:reg_size,
            qreg:  QRegister::new(reg_size),
            creg: CRegister::new(reg_size)
        }
    }

    // Initializes the Quantum Computer Simulator
    pub fn initialize(&mut self, init_state : Array<u64>) {
        self.qreg = QRegister::from_state(self.size, init_state);
    }    

    //
}