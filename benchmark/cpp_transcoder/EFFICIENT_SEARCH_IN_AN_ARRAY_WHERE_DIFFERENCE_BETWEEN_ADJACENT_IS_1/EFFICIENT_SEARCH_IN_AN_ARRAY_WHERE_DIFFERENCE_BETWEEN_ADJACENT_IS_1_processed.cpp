
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n, int x ) {
  int i = 0;
  while ( i <= n - 1 ) {
    if ( arr [ i ] == x ) return i;
    i += abs ( arr [ i ] - x );
  }
  return - 1;
}


