
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int x, int y, int n ) {
  int dp [ n + 1 ];
  dp [ 0 ] = 0;
  dp [ 1 ] = 1;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    if ( i - 1 >= 0 and ! dp [ i - 1 ] ) dp [ i ] = 1;
    else if ( i - x >= 0 and ! dp [ i - x ] ) dp [ i ] = 1;
    else if ( i - y >= 0 and ! dp [ i - y ] ) dp [ i ] = 1;
    else dp [ i ] = 0;
  }
  return dp [ n ];
}


