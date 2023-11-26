
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int low, int high ) {
  int max = arr [ low ];
  int i;
  for ( i = low + 1;
  i <= high;
  i ++ ) {
    if ( arr [ i ] > max ) max = arr [ i ];
    else break;
  }
  return max;
}


