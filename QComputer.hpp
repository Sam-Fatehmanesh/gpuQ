#include <arrayfire.h>
#include <vector>
#include "gates.hpp"
#include <iostream>


using af::array;
using af::seq;
using af::span;
using std::cout;

class QComputer {
private:
    array q_tensor;
    int qubit_count;
    int state_count;
    array c_reg;
public:
    QComputer() {
        
    }
};
/*
array kron(array &a, array &b) {
    af::dim4 neo_dims = a.dims() * b.dims();
    array product = array(neo_dims);
    af_print(a(3/b.dims().dims[0], span));
    cout << 3/b.dims().dims[0] << std::endl;
    gfor(seq i,neo_dims.dims[0]) {
        product(i,span) =
                af::transpose(
                af::flat(
                        af::matmul(
                                af::transpose(a(i/b.dims().dims[0],span)),
                                b(i%b.dims().dims[0],span))));
    }

    return product;
}*/
array kron(array &a, array &b) {
    array product = array(a.dims() * b.dims());

    int column_count = int(product.dims().dims[1]);
    double row_count = double (product.dims().dims[0]); 
    double columsreci = pow(double(column_count), -1.0);

    array rows = seq(double(b.dims().dims[0]));
    array columns = seq(double(b.dims().dims[1]));



    gfor(seq i, a.dims().elements()) {
        product(rows + (rows * (i / column_count)), columns + (columns * (i - (column_count * (i / column_count))))) =
                23;
    }

    return product;
}