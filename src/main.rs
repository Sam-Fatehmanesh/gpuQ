use arrayfire::*;
use num::complex::*;
use computer::*;
mod gates;
pub mod registers;
mod computer;

/*
struct Qubit {
    vector: Array<c32>,
}
impl Qubit {
    fn base() -> Qubit {
        let initarr :[c32; 2] = [Complex::new(1.0, 0.0),Complex::new(0.0, 0.0)];
        let dims = Dim4::new(&[2, 1, 1, 1]);

        return Qubit {vector: Array::new(&initarr,dims)};
    }
    fn new(vector: [c32; 2]) -> Qubit {
        let dims = Dim4::new(&[2, 1, 1, 1]);

        return Qubit {vector: Array::new(&vector,dims)};
    }
    fn print(self) {
        af_print!("qubit vector: ", self.vector);
    }
}*/

fn main() {
    set_backend(Backend::CUDA);
    /*println!("There are {:?} available backends", get_backend_count());
    let available = get_available_backends();
    if available.contains(&Backend::CPU) {
        println!("Evaluating CPU Backend...");
        set_backend(Backend::CPU);
        println!("There are {} CPU compute devices", device_count());
    }
    if available.contains(&Backend::CUDA ) {
        println!("Evaluating CUDA Backend...");
        set_backend(Backend::CUDA);
        println!("There are {} CUDA compute devices", device_count());
    }
    if available.contains(&Backend::OPENCL) {
        println!("Evaluating OpenCL Backend...");
        set_backend(Backend::OPENCL);
        println!("There are {} OpenCL compute devices", device_count());
    }*/



}
