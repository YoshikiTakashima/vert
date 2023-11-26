
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n, int k ) {
  if ( k > n ) return - 1;
  int sum = arr [ 0 ];
  for ( int i = 1;
  i < k;
  i ++ ) sum += arr [ i ];
  int max_sum = sum, max_end = k - 1;
  for ( int i = k;
  i < n;
  i ++ ) {
    int sum = sum + arr [ i ] - arr [ i - k ];
    if ( sum > max_sum ) {
      max_sum = sum;
      max_end = i;
    }
  }
  return max_end - k + 1;
}


