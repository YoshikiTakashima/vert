
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr1 [ ], int arr2 [ ], int m, int n ) {
  int i = 0;
  int j = 0;
  for ( i = 0;
  i < n;
  i ++ ) {
    for ( j = 0;
    j < m;
    j ++ ) {
      if ( arr2 [ i ] == arr1 [ j ] ) break;
    }
    if ( j == m ) return 0;
  }
  return 1;
}


