
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  int one_count = 0, zero_count = 0;
  int n = str . length ( );
  for ( int i = 0;
  i < n;
  i ++ ) if ( str [ i ] == '1' ) one_count ++;
  else zero_count ++;
  if ( one_count % 2 == 0 ) return zero_count;
  return one_count;
}


