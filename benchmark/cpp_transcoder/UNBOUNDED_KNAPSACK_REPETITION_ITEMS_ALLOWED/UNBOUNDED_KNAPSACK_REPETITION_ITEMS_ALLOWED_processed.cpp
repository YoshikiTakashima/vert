
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int W, int n, int val [ ], int wt [ ] ) {
  int dp [ W + 1 ];
  memset ( dp, 0, sizeof dp );
  int ans = 0;
  for ( int i = 0;
  i <= W;
  i ++ ) for ( int j = 0;
  j < n;
  j ++ ) if ( wt [ j ] <= i ) dp [ i ] = max ( dp [ i ], dp [ i - wt [ j ] ] + val [ j ] );
  return dp [ W ];
}


