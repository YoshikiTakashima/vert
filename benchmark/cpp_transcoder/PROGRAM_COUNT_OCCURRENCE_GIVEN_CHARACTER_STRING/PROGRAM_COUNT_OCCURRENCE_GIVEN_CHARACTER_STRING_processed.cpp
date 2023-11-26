
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string s, char c ) {
  int res = 0;
  for ( int i = 0;
  i < s . length ( );
  i ++ ) if ( s [ i ] == c ) res ++;
  return res;
}


