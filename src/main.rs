use arrayfire::*;
use computer::*;
mod gates;
mod computer;


fn main() {
    arrayfire::info();
    set_backend(Backend::CPU);

    let arr0: Array<c32> = flat(&identity(dim4!(3,3)));
    let arr1: Array<c32> = flat(&identity(dim4!(2,2)));

    let out = vector_outer_product(&arr0,&arr1);//vector_outer_product(&init_values, &init_values2);

    //let out = matrix2x2_fat_mul(&init_values, &init_values2);

    print(&out);

    /*
    let arr1 = constant(complex(2.0,0.0),dim4!(2));

    let arr2 = constant(complex(2.0,0.0),dim4!(2));

    let out = vector_outer_mul2(&arr1, &arr2);
    print(&out);*/

    //let mut test:Array<f32> = range(dim4!(5),0);
    //test = tile(&test, dim4!(2));

    //print(&test);
}