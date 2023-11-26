
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int nCr = 1, res = 1;
  for ( int r = 1;
  r <= n;
  r ++ ) {
    nCr = ( nCr * ( n + 1 - r ) ) / r;
    res += nCr * nCr;
  }
  return res;
}


