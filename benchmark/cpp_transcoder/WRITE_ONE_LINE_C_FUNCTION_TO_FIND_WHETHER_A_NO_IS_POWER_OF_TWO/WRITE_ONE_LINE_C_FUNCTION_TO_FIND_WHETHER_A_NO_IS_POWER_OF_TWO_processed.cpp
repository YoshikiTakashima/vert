
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  if ( n == 0 ) return 0;
  while ( n != 1 ) {
    if ( n % 2 != 0 ) return 0;
    n = n / 2;
  }
  return 1;
}


