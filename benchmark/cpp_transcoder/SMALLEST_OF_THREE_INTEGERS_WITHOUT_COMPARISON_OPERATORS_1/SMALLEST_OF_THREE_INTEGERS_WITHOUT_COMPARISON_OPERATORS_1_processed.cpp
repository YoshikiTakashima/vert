
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int x, int y, int z ) {
  if ( ! ( y / x ) ) return ( ! ( y / z ) ) ? y : z;
  return ( ! ( x / z ) ) ? x : z;
}


