
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a [ ], int n ) {
  int result = 1;
  for ( int i = 1;
  i <= n;
  ++ i ) {
    long long y = ( i * ( i + 1 ) ) / 2;
    if ( y < n ) result = i;
    else break;
  }
  return result;
}


