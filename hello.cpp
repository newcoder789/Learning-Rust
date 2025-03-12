#include<iostream>
using namespace std;

struct rect {
    int length;
    int width;
    int area(){
        return length*width;
    }
};
int main() {
    rect r1 ;
    r1.length = 5;
    r1.width = 3;
    cout << r1.area();
    return 0;
}