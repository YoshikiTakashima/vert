
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
unsigned int f_gold ( unsigned int n ) {
  unsigned int count = 0;
  while ( n ) {
    count += n & 1;
    n >>= 1;
  }
  return count;
}


