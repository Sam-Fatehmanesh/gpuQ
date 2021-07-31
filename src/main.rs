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

    let mut states:&[&[c32]] = &[
        &[complex(1.0,0.0),complex(0.0,0.0)],
        &[complex(0.0,0.0),complex(1.0,0.0)]];

    let mut states2:&[&[c32]] = &[
        &[complex(1.0,0.0),complex(0.0,0.0)],
        &[complex(0.0,0.0),complex(1.0,0.0)]];
    /*
    let mut init_values: Array<c32> = Array::new(&states[0], dim4!(2));
    let mut init_values2: Array<c32> = Array::new(&states2[0], dim4!(2));

    for i in 1..states.len() {
        init_values = join(1, &init_values, &Array::new(&states[i], dim4!(2)));
        init_values2 = join(1, &init_values2, &Array::new(&states2[i], dim4!(2)));
    }
    print(&init_values);
    print(&init_values2);
    let out = matrix2x2_fat_mul(&init_values,&init_values2);
    print(&out);*/

    //let mut qcompu = QCsim::new();
    //qcompu.init(states);

    //qcompu.addzero();



    //qcompu.print_state_vector();



}
