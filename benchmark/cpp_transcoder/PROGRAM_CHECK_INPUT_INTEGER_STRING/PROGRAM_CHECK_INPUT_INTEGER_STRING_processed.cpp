
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string s ) {
  for ( int i = 0;
  i < s . length ( );
  i ++ ) if ( isdigit ( s [ i ] ) == 0 ) return 0;
  return 1;
}


