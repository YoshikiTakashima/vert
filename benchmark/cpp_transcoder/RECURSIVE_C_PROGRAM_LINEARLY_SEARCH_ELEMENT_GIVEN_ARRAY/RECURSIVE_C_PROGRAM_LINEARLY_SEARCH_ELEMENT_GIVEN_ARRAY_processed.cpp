
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int l, int r, int x ) {
  if ( r < l ) return - 1;
  if ( arr [ l ] == x ) return l;
  if ( arr [ r ] == x ) return r;
  return f_gold ( arr, l + 1, r - 1, x );
}


