
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int high [ ], int low [ ], int n ) {
  if ( n <= 0 ) return 0;
  return max ( high [ n - 1 ] + f_gold ( high, low, ( n - 2 ) ), low [ n - 1 ] + f_gold ( high, low, ( n - 1 ) ) );
}


