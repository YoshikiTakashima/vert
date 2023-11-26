
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string isbn ) {
  int n = isbn . length ( );
  if ( n != 10 ) return 0;
  int sum = 0;
  for ( int i = 0;
  i < 9;
  i ++ ) {
    int digit = isbn [ i ] - '0';
    if ( 0 > digit || 9 < digit ) return 0;
    sum += ( digit * ( 10 - i ) );
  }
  char last = isbn [ 9 ];
  if ( last != 'X' && ( last < '0' || last > '9' ) ) return 0;
  sum += ( ( last == 'X' ) ? 10 : ( last - '0' ) );
  return ( sum % 11 == 0 );
}


