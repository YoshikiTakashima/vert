
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n, int k ) {
  int result = INT_MAX;
  sort ( arr, arr + n );
  for ( int i = 0;
  i <= n - k;
  i ++ ) result = min ( result, arr [ i + k - 1 ] - arr [ i ] );
  return result;
}


