
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int S [ ], int n ) {
  bool found = 0;
  sort ( S, S + n );
  for ( int i = n - 1;
  i >= 0;
  i -- ) {
    for ( int j = 0;
    j < n;
    j ++ ) {
      if ( i == j ) continue;
      for ( int k = j + 1;
      k < n;
      k ++ ) {
        if ( i == k ) continue;
        for ( int l = k + 1;
        l < n;
        l ++ ) {
          if ( i == l ) continue;
          if ( S [ i ] == S [ j ] + S [ k ] + S [ l ] ) {
            found = 1;
            return S [ i ];
          }
        }
      }
    }
  }
  if ( found == 0 ) return INT_MIN;
}


