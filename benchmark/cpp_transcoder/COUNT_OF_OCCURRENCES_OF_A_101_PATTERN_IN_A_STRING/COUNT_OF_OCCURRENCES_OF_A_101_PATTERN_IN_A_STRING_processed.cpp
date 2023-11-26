
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  int len = str . size ( );
  bool oneSeen = 0;
  int count = 0;
  for ( int i = 0;
  i < len;
  i ++ ) {
    if ( str [ i ] == '1' && oneSeen == 1 ) if ( str [ i - 1 ] == '0' ) count ++;
    if ( str [ i ] == '1' && oneSeen == 0 ) {
      oneSeen = 1;
      continue;
    }
    if ( str [ i ] != '0' && str [ i ] != '1' ) oneSeen = 0;
  }
  return count;
}


