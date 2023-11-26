
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int low, int high, int x ) {
  int mid;
  if ( x <= arr [ low ] ) return low;
  if ( x > arr [ high ] ) return - 1;
  mid = ( low + high ) / 2;
  if ( arr [ mid ] == x ) return mid;
  else if ( arr [ mid ] < x ) {
    if ( mid + 1 <= high && x <= arr [ mid + 1 ] ) return mid + 1;
    else return f_gold ( arr, mid + 1, high, x );
  }
  else {
    if ( mid - 1 >= low && x > arr [ mid - 1 ] ) return mid;
    else return f_gold ( arr, low, mid - 1, x );
  }
}


