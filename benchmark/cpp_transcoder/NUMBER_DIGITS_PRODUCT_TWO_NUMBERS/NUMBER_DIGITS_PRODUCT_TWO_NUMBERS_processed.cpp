
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a, int b ) {
  int count = 0;
  int p = abs ( a * b );
  if ( p == 0 ) return 1;
  while ( p > 0 ) {
    count ++;
    p = p / 10;
  }
  return count;
}


