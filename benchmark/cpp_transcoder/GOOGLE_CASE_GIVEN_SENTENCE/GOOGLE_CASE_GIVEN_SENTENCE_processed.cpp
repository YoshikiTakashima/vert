
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
string f_gold ( string s ) {
  int n = s . length ( );
  s [ 0 ] = tolower ( s [ 0 ] );
  for ( int i = 1;
  i < n;
  i ++ ) {
    if ( s [ i ] == ' ' && i < n ) {
      s [ i + 1 ] = tolower ( s [ i + 1 ] );
      i ++;
    }
    else s [ i ] = toupper ( s [ i ] );
  }
  return s;
}


