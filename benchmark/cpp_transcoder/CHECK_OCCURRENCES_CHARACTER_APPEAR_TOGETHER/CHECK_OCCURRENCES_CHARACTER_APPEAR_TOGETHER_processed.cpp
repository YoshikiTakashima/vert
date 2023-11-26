
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string s, char c ) {
  bool oneSeen = 0;
  int i = 0, n = s . length ( );
  while ( i < n ) {
    if ( s [ i ] == c ) {
      if ( oneSeen == 1 ) return 0;
      while ( i < n && s [ i ] == c ) i ++;
      oneSeen = 1;
    }
    else i ++;
  }
  return 1;
}


