
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a [ ], int b [ ], int n ) {
  sort ( a, a + n );
  sort ( b, b + n );
  int result = 0;
  for ( int i = 0;
  i < n;
  ++ i ) {
    result = result + abs ( a [ i ] - b [ i ] );
  }
  return result;
}


