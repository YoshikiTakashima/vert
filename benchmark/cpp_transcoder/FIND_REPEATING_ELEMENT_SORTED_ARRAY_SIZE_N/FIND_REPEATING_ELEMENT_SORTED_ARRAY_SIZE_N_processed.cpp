
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int low, int high ) {
  if ( low > high ) return - 1;
  int mid = ( low + high ) / 2;
  if ( arr [ mid ] != mid + 1 ) {
    if ( mid > 0 && arr [ mid ] == arr [ mid - 1 ] ) return mid;
    return f_gold ( arr, low, mid - 1 );
  }
  return f_gold ( arr, mid + 1, high );
}


