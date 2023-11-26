
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n, int x, int y ) {
  vector < bool > arr ( n + 1, 0 );
  if ( x <= n ) arr [ x ] = 1;
  if ( y <= n ) arr [ y ] = 1;
  int result = 0;
  for ( int i = min ( x, y );
  i <= n;
  i ++ ) {
    if ( arr [ i ] ) {
      if ( i + x <= n ) arr [ i + x ] = 1;
      if ( i + y <= n ) arr [ i + y ] = 1;
      result ++;
    }
  }
  return result;
}


