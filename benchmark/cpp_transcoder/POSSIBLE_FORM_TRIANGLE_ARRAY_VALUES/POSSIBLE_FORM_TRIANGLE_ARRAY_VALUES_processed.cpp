
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int N ) {
  if ( N < 3 ) return 0;
  sort ( arr, arr + N );
  for ( int i = 0;
  i < N - 2;
  i ++ ) if ( arr [ i ] + arr [ i + 1 ] > arr [ i + 2 ] ) return 1;
}


