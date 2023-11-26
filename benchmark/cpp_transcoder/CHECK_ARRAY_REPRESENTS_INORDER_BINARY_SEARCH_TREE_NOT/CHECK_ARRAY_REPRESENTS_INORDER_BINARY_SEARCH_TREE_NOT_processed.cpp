
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  if ( n == 0 || n == 1 ) return 1;
  for ( int i = 1;
  i < n;
  i ++ ) if ( arr [ i - 1 ] > arr [ i ] ) return 0;
  return 1;
}


