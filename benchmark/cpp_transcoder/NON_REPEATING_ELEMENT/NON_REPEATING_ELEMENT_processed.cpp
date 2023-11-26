
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  for ( int i = 0;
  i < n;
  i ++ ) {
    int j;
    for ( j = 0;
    j < n;
    j ++ ) if ( i != j && arr [ i ] == arr [ j ] ) break;
    if ( j == n ) return arr [ i ];
  }
  return - 1;
}


