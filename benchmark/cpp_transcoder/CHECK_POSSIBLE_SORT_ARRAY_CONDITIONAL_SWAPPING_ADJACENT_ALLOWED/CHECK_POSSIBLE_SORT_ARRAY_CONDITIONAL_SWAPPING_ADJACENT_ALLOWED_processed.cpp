
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  for ( int i = 0;
  i < n - 1;
  i ++ ) {
    if ( arr [ i ] > arr [ i + 1 ] ) {
      if ( arr [ i ] - arr [ i + 1 ] == 1 ) swap ( arr [ i ], arr [ i + 1 ] );
      else return 0;
    }
  }
  return 1;
}


