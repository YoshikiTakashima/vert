
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n, int idx ) {
  int result = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( arr [ i ] < arr [ idx ] ) result ++;
    if ( arr [ i ] == arr [ idx ] && i < idx ) result ++;
  }
  return result;
}


