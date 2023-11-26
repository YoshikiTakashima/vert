
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  int n = str . length ( );
  int i;
  for ( i = 0;
  i < n;
  i ++ ) if ( str [ i ] != 'a' ) break;
  if ( i * 2 != n ) return 0;
  int j;
  for ( j = i;
  j < n;
  j ++ ) if ( str [ j ] != 'b' ) return 0;
  return 1;
}


