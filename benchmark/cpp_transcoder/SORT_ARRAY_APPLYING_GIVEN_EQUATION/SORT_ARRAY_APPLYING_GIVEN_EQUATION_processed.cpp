
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
void f_gold ( int arr [ ], int n, int A, int B, int C ) {
  for ( int i = 0;
  i < n;
  i ++ ) arr [ i ] = A * arr [ i ] * arr [ i ] + B * arr [ i ] + C;
  int index, maximum = INT_MIN;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( maximum < arr [ i ] ) {
      index = i;
      maximum = arr [ i ];
    }
  }
  int i = 0, j = n - 1;
  int new_arr [ n ], k = 0;
  while ( i < index && j > index ) {
    if ( arr [ i ] < arr [ j ] ) new_arr [ k ++ ] = arr [ i ++ ];
    else new_arr [ k ++ ] = arr [ j -- ];
  }
  while ( i < index ) new_arr [ k ++ ] = arr [ i ++ ];
  while ( j > index ) new_arr [ k ++ ] = arr [ j -- ];
  new_arr [ n - 1 ] = maximum;
  for ( int i = 0;
  i < n;
  i ++ ) arr [ i ] = new_arr [ i ];
}


