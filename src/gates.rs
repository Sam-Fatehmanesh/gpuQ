use arrayfire::*;
use num::complex::Complex32;
use crate::computer::complex;
use std::intrinsics::sqrtf32;

pub struct GateGenerator {
    pub hadamard: Array<Complex32>,
    //pub ry: Array<Complex32>,
    pub identity: Array<Complex32>
}
impl GateGenerator {
    pub fn new() -> GateGenerator{
        let mut H: Array<c32> = identity(dim4!(2,2)) * complex(1/(2.0f32.sqrt()),0.0);

        GateGenerator{
            hadamard: H,
            identity: identity(dim4!(2,2))
        }

    }
}