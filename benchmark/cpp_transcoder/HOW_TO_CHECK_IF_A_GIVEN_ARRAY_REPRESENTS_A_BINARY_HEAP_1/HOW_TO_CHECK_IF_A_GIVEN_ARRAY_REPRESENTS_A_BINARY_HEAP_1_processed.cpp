
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  for ( int i = 0;
  i <= ( n - 2 ) / 2;
  i ++ ) {
    if ( arr [ 2 * i + 1 ] > arr [ i ] ) return 0;
    if ( 2 * i + 2 < n && arr [ 2 * i + 2 ] > arr [ i ] ) return 0;
  }
  return 1;
}


