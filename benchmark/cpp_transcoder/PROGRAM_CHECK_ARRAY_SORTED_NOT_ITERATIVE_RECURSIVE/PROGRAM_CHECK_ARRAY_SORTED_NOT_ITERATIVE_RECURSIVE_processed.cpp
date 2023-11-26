
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  if ( n == 1 || n == 0 ) return 1;
  if ( arr [ n - 1 ] < arr [ n - 2 ] ) return 0;
  return f_gold ( arr, n - 1 );
}


