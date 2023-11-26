
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  if ( n == 0 || n == 9 ) return 1;
  if ( n < 9 ) return 0;
  return f_gold ( ( int ) ( n >> 3 ) - ( int ) ( n & 7 ) );
}


