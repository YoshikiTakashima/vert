
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string s ) {
  int n = s . length ( );
  for ( int i = 1;
  i < n;
  i ++ ) if ( s [ i ] != s [ 0 ] ) return 0;
  return 1;
}


