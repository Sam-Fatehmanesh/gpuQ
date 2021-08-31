use arrayfire::*;
use crate::computer::complex;

pub struct GateGenerator {
    pub hadamard: Array<c32>,
    //pub ry: Array<Complex32>,
    pub identity: Array<c32>
}
impl GateGenerator {
    pub fn new() -> GateGenerator{
        let two: f32 = 2.0;
        let identity_m: Array<c32> = identity(dim4!(2,2));
        let H: Array<c32> = &identity_m * complex(1.0/two.sqrt(), 0.0);
        GateGenerator{
            hadamard: H,
            identity: identity_m
        }

    }
}