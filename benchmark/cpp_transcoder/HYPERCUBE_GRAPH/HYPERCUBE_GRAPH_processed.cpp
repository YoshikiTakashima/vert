
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  if ( n == 1 ) return 2;
  return 2 * f_gold ( n - 1 );
}


