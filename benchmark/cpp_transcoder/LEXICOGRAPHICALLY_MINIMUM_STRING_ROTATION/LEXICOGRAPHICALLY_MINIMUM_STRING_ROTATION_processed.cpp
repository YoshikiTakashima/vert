
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
string f_gold ( string str ) {
  int n = str . length ( );
  string arr [ n ];
  string concat = str + str;
  for ( int i = 0;
  i < n;
  i ++ ) arr [ i ] = concat . substr ( i, n );
  sort ( arr, arr + n );
  return arr [ 0 ];
}


