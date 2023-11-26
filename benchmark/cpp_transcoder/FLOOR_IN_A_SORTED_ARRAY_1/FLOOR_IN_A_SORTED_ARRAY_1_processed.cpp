
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int low, int high, int x ) {
  if ( low > high ) return - 1;
  if ( x >= arr [ high ] ) return high;
  int mid = ( low + high ) / 2;
  if ( arr [ mid ] == x ) return mid;
  if ( mid > 0 && arr [ mid - 1 ] <= x && x < arr [ mid ] ) return mid - 1;
  if ( x < arr [ mid ] ) return f_gold ( arr, low, mid - 1, x );
  return f_gold ( arr, mid + 1, high, x );
}


