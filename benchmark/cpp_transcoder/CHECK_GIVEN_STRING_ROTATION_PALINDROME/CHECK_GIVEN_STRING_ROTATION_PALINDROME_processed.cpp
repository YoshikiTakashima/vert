
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  int l = 0;
  int h = str . length ( ) - 1;
  while ( h > l ) if ( str [ l ++ ] != str [ h -- ] ) return 0;
  return 1;
}


