
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int low, int high ) {
  if ( high >= low ) {
    int mid = ( low + high ) / 2;
    if ( mid == arr [ mid ] ) return mid;
    if ( mid > arr [ mid ] ) return f_gold ( arr, ( mid + 1 ), high );
    else return f_gold ( arr, low, ( mid - 1 ) );
  }
  return - 1;
}


