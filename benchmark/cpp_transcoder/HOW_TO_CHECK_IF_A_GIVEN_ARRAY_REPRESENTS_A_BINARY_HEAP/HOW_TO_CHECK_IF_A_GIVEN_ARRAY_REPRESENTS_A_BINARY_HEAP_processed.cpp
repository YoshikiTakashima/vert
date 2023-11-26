
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int i, int n ) {
  if ( i > ( n - 2 ) / 2 ) return 1;
  if ( arr [ i ] >= arr [ 2 * i + 1 ] && arr [ i ] >= arr [ 2 * i + 2 ] && f_gold ( arr, 2 * i + 1, n ) && f_gold ( arr, 2 * i + 2, n ) ) return 1;
  return 0;
}


