
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int position = 1;
  int m = 1;
  while ( ! ( n & m ) ) {
    m = m << 1;
    position ++;
  }
  return position;
}


