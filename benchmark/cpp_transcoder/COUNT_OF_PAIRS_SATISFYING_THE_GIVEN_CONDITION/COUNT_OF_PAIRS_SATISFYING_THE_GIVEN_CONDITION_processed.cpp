
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a, int b ) {
  string s = to_string ( b );
  int i;
  for ( i = 0;
  i < s . length ( );
  i ++ ) {
    if ( s [ i ] != '9' ) break;
  }
  int result;
  if ( i == s . length ( ) ) result = a * s . length ( );
  else result = a * ( s . length ( ) - 1 );
  return result;
}


