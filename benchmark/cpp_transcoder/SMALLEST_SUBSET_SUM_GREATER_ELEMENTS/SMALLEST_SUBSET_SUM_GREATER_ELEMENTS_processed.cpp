
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  int halfSum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) halfSum = halfSum + arr [ i ];
  halfSum = halfSum / 2;
  sort ( arr, arr + n, greater < int > ( ) );
  int res = 0, curr_sum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    curr_sum += arr [ i ];
    res ++;
    if ( curr_sum > halfSum ) return res;
  }
  return res;
}


