
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
void f_gold ( char s1 [ ], char s2 [ ], int index = 0 ) {
  s2 [ index ] = s1 [ index ];
  if ( s1 [ index ] == '\0' ) return;
  f_gold ( s1, s2, index + 1 );
}


