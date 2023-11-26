
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n, int a [ ] ) {
  int res = 0;
  for ( int i = 0;
  i < 2 * n;
  i ++ ) {
    if ( i % 2 == 0 ) res += a [ i ] * a [ i ];
    else res -= a [ i ] * a [ i ];
  }
  return res;
}


