
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int dp [ n + 1 ];
  dp [ 0 ] = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) {
    if ( i >= 1 && i <= 3 ) dp [ i ] = 1;
    else if ( i == 4 ) dp [ i ] = 2;
    else dp [ i ] = dp [ i - 1 ] + dp [ i - 4 ];
  }
  return dp [ n ];
}


