
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int left, int right ) {
  if ( left <= right ) {
    int mid = ( left + right ) / 2;
    if ( arr [ mid - 1 ] < arr [ mid ] && arr [ mid ] > arr [ mid + 1 ] ) return mid;
    if ( arr [ mid ] < arr [ mid + 1 ] ) return f_gold ( arr, mid + 1, right );
    else return f_gold ( arr, left, mid - 1 );
  }
  return - 1;
}


