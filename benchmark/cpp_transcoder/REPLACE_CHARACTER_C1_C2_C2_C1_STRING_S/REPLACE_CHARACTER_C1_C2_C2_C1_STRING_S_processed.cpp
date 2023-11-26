
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
string f_gold ( string s, char c1, char c2 ) {
  int l = s . length ( );
  for ( int i = 0;
  i < l;
  i ++ ) {
    if ( s [ i ] == c1 ) s [ i ] = c2;
    else if ( s [ i ] == c2 ) s [ i ] = c1;
  }
  return s;
}


