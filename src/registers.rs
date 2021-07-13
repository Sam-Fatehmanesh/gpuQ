use arrayfire::af_print;
use arrayfire::Array;
use arrayfire::pow;
use arrayfire::dim4;
use arrayfire::eval;
use arrayfire::Dim4;
use arrayfire::constant;
use arrayfire::*;
use arrayfire::c32;
use core::num::*;
use num::Complex;
use std::vec::Vec;

pub struct QRegister {
    pub collapsed: bool,
    size: u64,
    states: u64,
    pub tensor: Array<c32>
}
impl QRegister {
    // generates a blank quantum registry
    pub fn new(sizeofreg: u64) -> QRegister {
        let posstates = u64::pow(2,sizeofreg as u32);
        QRegister {
            collapsed: false,
            size: sizeofreg,
            states: posstates,
            tensor: constant(Complex::new(0.0, 0.0),Dim4::new(&[posstates,1,1,1]))
        }
    } 

    // generates a quantum registry from a classical registry
    pub fn from_classical(creg :CRegister) -> QRegister {
        // makes a copy of the size of the classical registry
        let creg_size = creg.size;
        //collects the state of the CReg
        let state = creg.state();
        // generates a blank quantum registry
        let mut qreg = constant(Complex::new(0.0, 0.0), Dim4::new(&[u64::pow(2 as u64,creg_size as u32),1,1,1]));
        // generates a single value Array of complex value 1.0 + 0i
        let c_one = constant(Complex::new(1.0, 0.0), dim4!(1));
        // sets the quantum reg at the index of state to c_one
        eval!(qreg[state] = c_one);
        QRegister {
            collapsed: false,
            size: creg_size,
            states: u64::pow(2, creg_size as u32),
            tensor: qreg
        }
    }

    // generates a quantum registry from a state
    pub fn from_state(size : u64, init_state : Array<u64>) -> QRegister {
        let state = init_state;
        // generates a blank quantum registry
        let mut qreg = constant(Complex::new(0.0, 0.0), Dim4::new(&[u64::pow(2 as u64,size as u32),1,1,1]));
        // generates a single value Array of complex value 1.0 + 0i
        let c_one = constant(Complex::new(1.0, 0.0), dim4!(1));
        // sets the quantum reg at the index of state to c_one
        eval!(qreg[state] = c_one);
        QRegister {
            collapsed: false,
            size: size,
            states: u64::pow(2, size as u32),
            tensor: qreg
        }
    }
}

#[derive(Clone)]
pub struct CRegister {
    size: u64,
    pub tensor: Array<u64>
}
impl CRegister {
    // generates a blank classical registry
    pub fn new(sizeofreg: u64) -> CRegister {  
        CRegister{
            size: sizeofreg,
            tensor: constant(0 as u64, dim4!(sizeofreg))
        }
    }

    //generates a state value from the classical registary, identical to converting from binary to decimal
    pub fn state(&self) -> Array<u64> {
        // generates a sequencial tensor [0,1,2...]
        let mut seq_arr:Array<u64> = arrayfire::range(dim4!(self.size),0);
        // rasies 2 to the power of each value in seqarr [2^0,2^1,2^2...] 
        // then multiplies it by the Creg tensor 
        seq_arr = &self.tensor * arrayfire::pow(&constant(2 as u64, dim4!(1)),&seq_arr,false);
        //sums up the remaining powers of two and returns
        arrayfire::sum(&seq_arr,0)
    }
}