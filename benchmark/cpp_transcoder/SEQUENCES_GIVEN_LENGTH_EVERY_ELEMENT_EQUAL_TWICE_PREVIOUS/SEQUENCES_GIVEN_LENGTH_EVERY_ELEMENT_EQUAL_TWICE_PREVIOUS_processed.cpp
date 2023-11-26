
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int m, int n ) {
  if ( m < n ) return 0;
  if ( n == 0 ) return 1;
  return f_gold ( m - 1, n ) + f_gold ( m / 2, n - 1 );
}


