
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int low, int high ) {
  if ( high < low ) return 0;
  if ( high == low ) return low;
  int mid = low + ( high - low ) / 2;
  if ( mid < high && arr [ mid + 1 ] < arr [ mid ] ) return ( mid + 1 );
  if ( mid > low && arr [ mid ] < arr [ mid - 1 ] ) return mid;
  if ( arr [ high ] > arr [ mid ] ) return f_gold ( arr, low, mid - 1 );
  return f_gold ( arr, mid + 1, high );
}


