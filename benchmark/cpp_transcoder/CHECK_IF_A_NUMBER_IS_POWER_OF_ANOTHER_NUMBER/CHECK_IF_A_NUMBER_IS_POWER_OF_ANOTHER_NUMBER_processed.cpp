
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int x, int y ) {
  if ( x == 1 ) return ( y == 1 );
  int pow = 1;
  while ( pow < y ) pow *= x;
  return ( pow == y );
}


