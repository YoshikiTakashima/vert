
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n, int x ) {
  for ( int i = 0;
  i < n - 1;
  i ++ ) for ( int j = i + 1;
  i < n;
  i ++ ) if ( arr [ i ] * arr [ j ] == x ) return 1;
  return 0;
}


