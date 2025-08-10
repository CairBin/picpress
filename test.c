#include "include/picpress.h"

int main(){
    compress_img_c("t.jpg", "t1.jpeg", "jpeg", 10, 0, 0, RS_DEFAULT, 0);
    return 0;
}