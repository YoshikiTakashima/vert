
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n, int k ) {
  int dist_count = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    int j;
    for ( j = 0;
    j < n;
    j ++ ) if ( i != j && arr [ j ] == arr [ i ] ) break;
    if ( j == n ) dist_count ++;
    if ( dist_count == k ) return arr [ i ];
  }
  return - 1;
}


