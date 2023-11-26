
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string s ) {
  int l = s . length ( );
  int j;
  for ( int i = 0, j = l - 1;
  i <= j;
  i ++, j -- ) {
    if ( s [ i ] != s [ j ] ) return 0;
  }
  return 1;
}


