
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int l, int r, int x ) {
  if ( r >= l ) {
    int mid = l + ( r - l ) / 2;
    if ( arr [ mid ] == x ) return mid;
    if ( arr [ mid ] > x ) return f_gold ( arr, l, mid - 1, x );
    return f_gold ( arr, mid + 1, r, x );
  }
  return - 1;
}


