#include <arrayfire.h>
#include <vectors>

using namespace af;

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