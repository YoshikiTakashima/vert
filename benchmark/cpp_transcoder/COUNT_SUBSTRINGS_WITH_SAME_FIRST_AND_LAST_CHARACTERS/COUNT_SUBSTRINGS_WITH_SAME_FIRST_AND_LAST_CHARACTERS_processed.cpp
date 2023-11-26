
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string s ) {
  int result = 0;
  int n = s . length ( );
  for ( int i = 0;
  i < n;
  i ++ ) for ( int j = i;
  j < n;
  j ++ ) if ( s [ i ] == s [ j ] ) result ++;
  return result;
}


