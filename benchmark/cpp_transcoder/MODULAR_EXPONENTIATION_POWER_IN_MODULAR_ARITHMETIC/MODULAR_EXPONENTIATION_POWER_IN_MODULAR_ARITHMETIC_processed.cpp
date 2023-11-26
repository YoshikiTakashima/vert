
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int x, unsigned int y, int p ) {
  int res = 1;
  x = x % p;
  while ( y > 0 ) {
    if ( y & 1 ) res = ( res * x ) % p;
    y = y >> 1;
    x = ( x * x ) % p;
  }
  return res;
}


