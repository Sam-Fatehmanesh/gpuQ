use core::num::*;
use std::ptr::null;
use arrayfire::*;
use core::num::*;
use num::Complex;
use std::vec::Vec;

pub struct QCsim {
    q_tensor: Array<c32>,
    qubit_count: usize,
    state_count: usize,
    c_reg: Array<u32>,

}

impl QCsim {
    
    // Creates a new Quantum Computer Simulator Object
    pub fn new() -> QCsim {
        QCsim {
            q_tensor: Array::<c32>::new_empty(dim4!(1)),
            qubit_count: 0,
            state_count: 1,
            c_reg: Array::<u32>::new_empty(dim4!(1)),
        }
    }

    // initializes the QC
    pub fn init(&mut self, qubit_init: &[&[c32]]) {
        self.qubit_count = qubit_init.len();
        self.state_count = 2_i32.pow(self.qubit_count as u32) as usize;

        // loads qubit init data onto GPU
        let mut init_values: Array<c32> = Array::new(&qubit_init[0], dim4!(2));

        for i in 1..self.qubit_count {
            init_values = join(1, &init_values, &Array::new(&qubit_init[i], dim4!(2)));
        }

        // builds state vector from individual qubit states
        // initializes the q_tensor with the first qubit state
        self.q_tensor = view!(init_values[0:1:1,0:0:1]);
        // normalizes the first qubit
        self.q_tensor = normalize_q(&self.q_tensor);

        // does the same thing for each qubit adding each as provided by qubit_init
        let mut q_index:Seq<i32>;
        let state_index = Seq::new(0, 1, 1);
        for i in 1..qubit_init.len() {
            q_index = Seq::new(i as i32, i as i32, 1);
            self.addqubit_arr(&view!(init_values[state_index,q_index]));
        }
    }

    // adds qubits to the register
    pub fn addqubit(&mut self, state_zero: c32, state_one: c32) {
        if self.qubit_count == 0 {
            let start_qubit: &[&[c32]] = &[&[state_zero,state_one]];//Vec<Vec<c32>> = vec![vec![state_zero,state_one]];
            self.init(start_qubit);
            self.qubit_count += 1;
            self.state_count = self.state_count * 2;
            return;
        }
        let mut qubit_state = Array::new(&[state_zero,state_one],dim4!(2));
        qubit_state = normalize_q(&qubit_state);
        self.q_tensor = vector_fat_mul(&self.q_tensor, &qubit_state);
        self.qubit_count += 1;
        self.state_count = self.state_count * 2;
    }

    // adds qubits to the register can not run addqubit_arr when not initialized with func init
    pub fn addqubit_arr(&mut self, states: &Array<c32>) {
        if self.qubit_count == 0 {
            print!("ERROR: can not run addqubit_arr when not initialized with func init");
            return;
        }
        let qubit_state = normalize_q(&states);
        self.q_tensor = vector_fat_mul(&self.q_tensor, &qubit_state);
        self.qubit_count += 1;
        self.state_count = self.state_count * 2;
    }

    // adds qubit with zero state
    pub fn addzero(&mut self) {
     self.addqubit(complex(1.0,0.0),complex(0.0,0.0));
    }

    // apply a one qubit
    pub fn apply_single_gate(&mut self, index: &[i32], gate: Array<c32>) {
        if index.len() < 1 {

            return;
        }


    }

    // prints the quantum state vector
    pub fn print_state_vector(&self) {
        print(&self.q_tensor);
    }
}

// returns a complex number of type c32
pub fn complex(re: f32, im: f32) -> c32 {
    Complex::new(re,im)
}

// changes each value of the tensor to the value equal to 2 ^ the prior value
pub fn twoEx(ex: &Array<f32>) -> Array<f32>{
    arrayfire::pow(&2,ex,true)
}

// returns a tensor with each value changed to their reciprocal
pub fn reciprocal(base: &Array<f32>) -> Array<f32>{
    arrayfire::pow(base,&(-1),true)
}

// multiplies two vectors together in this manner [x1,x2] * [y1,y2] = [x1y1, x1y2, x2y1, x2y2]
pub fn vector_fat_mul(l: &Array<c32>, r: &Array<c32>) -> Array<c32> {
    // creates a matrix of 0s to combine with the r vector
    let size_zeros = constant(complex(0.0,0.0),dim4!(r.dims().get()[0],l.dims().get()[0]-1));
    // creates the matrix factor which is formed from joining the 0s with the r vector
    let mut matrix_factor = join(1,&r,&size_zeros);

    // begins the shiftadd matrix which is a shifted form of the initial matrix_factor
    let mut shiftadd: Array<c32> = shift(&matrix_factor,&[0,1,0,0]);

    // loops while adding progressively shifted shiftadd matrices to the matrix factor
    for i in 1..(l.dims().get()[0]) {
        matrix_factor = join(0,&matrix_factor,&shiftadd);
        shiftadd = shift(&shiftadd,&[0,1,0,0]);
    }

    //multiplies matrix add by l vector to get the output vector
    matmul(&matrix_factor, &l, MatProp::NONE, MatProp::NONE)
}

// multiplies two 2x2 matrices together in this manner [[x,y],[w,z]]*[[a,b],[c,d]]=[[xa,xb,ya,yb],[xc,xd,yc,yd],[wa,wb,za,zb],[wc,wd,zc,zd]]
pub fn matrix2x2_fat_mul(l: &Array<c32>, r: &Array<c32>) -> Array<c32> {
    // generates the required matrix to format and prepare
    let mut format_matrix = constant(complex(0.0,0.0),dim4!(4,2));
    let one = constant(complex(1.0,0.0),dim4!(1));
    eval!(format_matrix[0:0:1,0:0:1] = one);
    eval!(format_matrix[2:2:1,1:1:1] = one);

    // formats the factor matrices by multiplying them by
    let mut factor_matrix_l = matmul(&format_matrix, l, MatProp::NONE, MatProp::NONE);
    let mut factor_matrix_r = matmul(&format_matrix, r, MatProp::NONE, MatProp::NONE);

    // adds a shifted version of the factor matrix to itself
    factor_matrix_l = join(1, &factor_matrix_l, &shift(&factor_matrix_l, &[1, 0, 0, 0]));
    factor_matrix_r = join(1, &factor_matrix_r, &shift(&factor_matrix_r, &[1, 0, 0, 0]));

    // returns the matrix multiplication product from the multiplication of factor_matrix_l and factor_matrix_r
    matmul(&factor_matrix_l, &factor_matrix_r, MatProp::NONE, MatProp::NONE)
}

// multiplies two 2x2 matrices together in this manner [[x,y],[w,z]]*[[a,b],[c,d]]=[[xa,xb,ya,yb],[xc,xd,yc,yd],[wa,wb,za,zb],[wc,wd,zc,zd]]
pub fn matrix_fat_mul(l: &Array<c32>, r: &Array<c32>) -> Array<c32> {
    // generates the required matrix to format and prepare
    let mut format_matrix = constant(complex(0.0,0.0),dim4!(4,2));
    let one = constant(complex(1.0,0.0),dim4!(1));
    eval!(format_matrix[0:0:1,0:0:1] = one);
    
    eval!(format_matrix[2:2:1,1:1:1] = one);

    // formats the factor matrices by multiplying them by
    let mut factor_matrix_l = matmul(&format_matrix, l, MatProp::NONE, MatProp::NONE);
    let mut factor_matrix_r = matmul(&format_matrix, r, MatProp::NONE, MatProp::NONE);

    // adds a shifted version of the factor matrix to itself
    factor_matrix_l = join(1, &factor_matrix_l, &shift(&factor_matrix_l, &[1, 0, 0, 0]));
    factor_matrix_r = join(1, &factor_matrix_r, &shift(&factor_matrix_r, &[1, 0, 0, 0]));

    // returns the matrix multiplication product from the multiplication of factor_matrix_l and factor_matrix_r
    matmul(&factor_matrix_l, &factor_matrix_r, MatProp::NONE, MatProp::NONE)
}

// normalizes complex vectors such as quantum state vectors
pub fn normalize_q(state_vector: &Array<c32>) -> Array<c32> {
    // gets the sum of the probabilities of all the states
    let probability_sum = sum(&probability(state_vector), 0);
    // creates the factor to multiply the states by which would normalize the state vector
    let normalization_factor: Array<c32> = reciprocal(&sqrt(&probability_sum)).cast();
    // converts the normalization factor into a diagonal matrix so that normalization can
    // occur even with multiple quantum registers
    matmul(state_vector,
           &diag_create(&moddims(&normalization_factor,
                                 dim4!(normalization_factor.dims().get()[1],1)), 0),
           MatProp::NONE, MatProp::NONE)
}

// Returns the probability of each state of the quantum system
pub fn probability(state_vector: &Array<c32>) -> Array<f32> {
    twoEx(&abs(&state_vector.copy()))
}