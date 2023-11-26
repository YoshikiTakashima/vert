
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int ar [ ], int n ) {
  int res = 0;
  sort ( ar, ar + n );
  for ( int i = 0;
  i < n;
  i ++ ) {
    int count = 1;
    for (;
    i < n - 1;
    i ++ ) {
      if ( ar [ i ] == ar [ i + 1 ] ) count ++;
      else break;
    }
    res = max ( res, count );
  }
  return res;
}


