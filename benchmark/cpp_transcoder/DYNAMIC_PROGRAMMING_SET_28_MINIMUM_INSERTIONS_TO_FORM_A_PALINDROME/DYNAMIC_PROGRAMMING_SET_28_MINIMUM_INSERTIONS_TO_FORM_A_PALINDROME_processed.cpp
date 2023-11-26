
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( char str [ ], int l, int h ) {
  if ( l > h ) return INT_MAX;
  if ( l == h ) return 0;
  if ( l == h - 1 ) return ( str [ l ] == str [ h ] ) ? 0 : 1;
  return ( str [ l ] == str [ h ] ) ? f_gold ( str, l + 1, h - 1 ) : ( min ( f_gold ( str, l, h - 1 ), f_gold ( str, l + 1, h ) ) + 1 );
}


