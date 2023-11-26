
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  int n = str . length ( );
  if ( ( str [ n - 1 ] - '0' ) % 2 != 0 ) return 0;
  int digitSum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) digitSum += ( str [ i ] - '0' );
  return ( digitSum % 3 == 0 );
}


