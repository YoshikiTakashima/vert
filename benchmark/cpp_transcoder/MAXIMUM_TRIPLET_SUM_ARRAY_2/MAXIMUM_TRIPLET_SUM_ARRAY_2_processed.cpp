
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  int maxA = INT_MIN, maxB = INT_MIN, maxC = INT_MIN;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( arr [ i ] > maxA ) {
      maxC = maxB;
      maxB = maxA;
      maxA = arr [ i ];
    }
    else if ( arr [ i ] > maxB ) {
      maxC = maxB;
      maxB = arr [ i ];
    }
    else if ( arr [ i ] > maxC ) maxC = arr [ i ];
  }
  return ( maxA + maxB + maxC );
}


