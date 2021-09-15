#include "QComputer.hpp"

int main(){
    af::info();
    array mat1 = af::identity(2,2);
    array mat2 = af::identity(2,2);

    array product = kron(mat1,mat2);

    af_print(product);

    return 0;
}