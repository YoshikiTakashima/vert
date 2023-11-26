
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
void f_gold ( int arr [ ], int l, int h ) {
  if ( l >= h ) return;
  if ( arr [ l ] > arr [ h ] ) swap ( arr [ l ], arr [ h ] );
  if ( h - l + 1 > 2 ) {
    int t = ( h - l + 1 ) / 3;
    f_gold ( arr, l, h - t );
    f_gold ( arr, l + t, h );
    f_gold ( arr, l, h - t );
  }
}


