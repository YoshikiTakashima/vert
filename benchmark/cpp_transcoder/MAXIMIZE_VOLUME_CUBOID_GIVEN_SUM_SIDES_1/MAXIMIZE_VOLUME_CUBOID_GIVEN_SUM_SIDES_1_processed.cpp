
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int s ) {
  int length = s / 3;
  s -= length;
  int breadth = s / 2;
  int height = s - breadth;
  return length * breadth * height;
}


