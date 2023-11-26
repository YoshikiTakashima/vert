
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string num ) {
  int n = num . length ( );
  if ( n == 0 && num [ 0 ] == '0' ) return 1;
  if ( n % 3 == 1 ) num = "00" + num;
  if ( n % 3 == 2 ) num = "0" + num;
  int gSum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    int group = 0;
    group += ( num [ i ++ ] - '0' ) * 100;
    group += ( num [ i ++ ] - '0' ) * 10;
    group += num [ i ] - '0';
    gSum += group;
  }
  if ( gSum > 1000 ) {
    num = to_string ( gSum );
    n = num . length ( );
    gSum = f_gold ( num );
  }
  return ( gSum == 999 );
}


