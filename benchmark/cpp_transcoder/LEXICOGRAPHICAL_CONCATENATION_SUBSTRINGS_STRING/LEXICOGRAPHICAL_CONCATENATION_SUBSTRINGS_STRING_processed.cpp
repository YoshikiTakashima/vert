
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
string f_gold ( string s ) {
  int n = s . length ( );
  int sub_count = n * ( n + 1 ) / 2;
  string arr [ sub_count ];
  int index = 0;
  for ( int i = 0;
  i < n;
  i ++ ) for ( int len = 1;
  len <= n - i;
  len ++ ) arr [ index ++ ] = s . substr ( i, len );
  sort ( arr, arr + sub_count );
  string res = "";
  for ( int i = 0;
  i < sub_count;
  i ++ ) res += arr [ i ];
  return res;
}


