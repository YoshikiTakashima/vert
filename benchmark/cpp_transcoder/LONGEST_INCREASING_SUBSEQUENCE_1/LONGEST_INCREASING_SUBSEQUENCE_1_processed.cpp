
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  int f_gold [ n ];
  f_gold [ 0 ] = 1;
  for ( int i = 1;
  i < n;
  i ++ ) {
    f_gold [ i ] = 1;
    for ( int j = 0;
    j < i;
    j ++ ) if ( arr [ i ] > arr [ j ] && f_gold [ i ] < f_gold [ j ] + 1 ) f_gold [ i ] = f_gold [ j ] + 1;
  }
  return * max_element ( f_gold, f_gold + n );
}


