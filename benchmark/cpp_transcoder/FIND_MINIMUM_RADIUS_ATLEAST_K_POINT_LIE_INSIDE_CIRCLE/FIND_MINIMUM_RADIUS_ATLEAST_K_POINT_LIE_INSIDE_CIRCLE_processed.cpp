
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int k, int x [ ], int y [ ], int n ) {
  int dis [ n ];
  for ( int i = 0;
  i < n;
  i ++ ) dis [ i ] = x [ i ] * x [ i ] + y [ i ] * y [ i ];
  sort ( dis, dis + n );
  return dis [ k - 1 ];
}


