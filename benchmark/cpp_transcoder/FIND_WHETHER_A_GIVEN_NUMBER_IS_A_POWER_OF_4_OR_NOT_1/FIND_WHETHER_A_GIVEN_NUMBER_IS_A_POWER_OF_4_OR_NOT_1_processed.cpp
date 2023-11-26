
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( unsigned int n ) {
  int count = 0;
  if ( n && ! ( n & ( n - 1 ) ) ) {
    while ( n > 1 ) {
      n >>= 1;
      count += 1;
    }
    return ( count % 2 == 0 ) ? 1 : 0;
  }
  return 0;
}


