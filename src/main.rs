use arrayfire::*;
use num::complex::*;
use computer::*;
mod gates;
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

    //let mut states:&[&[c32]] = &[
    //    &[complex(0.3,0.2),complex(0.5,0.4)],
    //    &[complex(0.9,0.0),complex(0.05,0.0)]];

    let mut qcompu = QCsim::new();
    //qcompu.init(states);

    qcompu.addzero();



    qcompu.print_state_vector();

}
