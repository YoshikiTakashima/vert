
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n, int m ) {
  if ( n > m ) return 1;
  bool DP [ m ];
  memset ( DP, 0, m );
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( DP [ 0 ] ) return 1;
    bool temp [ m ];
    memset ( temp, 0, m );
    for ( int j = 0;
    j < m;
    j ++ ) {
      if ( DP [ j ] == 1 ) {
        if ( DP [ ( j + arr [ i ] ) % m ] == 0 ) temp [ ( j + arr [ i ] ) % m ] = 1;
      }
    }
    for ( int j = 0;
    j < m;
    j ++ ) if ( temp [ j ] ) DP [ j ] = 1;
    DP [ arr [ i ] % m ] = 1;
  }
  return DP [ 0 ];
}


