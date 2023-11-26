
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
unsigned int f_gold ( unsigned int n ) {
  unsigned int p = 1;
  if ( n && ! ( n & ( n - 1 ) ) ) return n;
  while ( p < n ) p <<= 1;
  return p;
}


