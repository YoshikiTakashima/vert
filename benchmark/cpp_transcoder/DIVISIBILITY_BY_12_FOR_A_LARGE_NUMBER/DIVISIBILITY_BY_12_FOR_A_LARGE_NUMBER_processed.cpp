
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string num ) {
  if ( num . length ( ) >= 3 ) {
    int d1 = ( int ) num [ num . length ( ) - 1 ];
    if ( d1 % 2 != 0 ) return ( 0 );
    int d2 = ( int ) num [ num . length ( ) - 2 ];
    int sum = 0;
    for ( int i = 0;
    i < num . length ( );
    i ++ ) sum += num [ i ];
    return ( sum % 3 == 0 && ( d2 * 10 + d1 ) % 4 == 0 );
  }
  else {
    int number = stoi ( num );
    return ( number % 12 == 0 );
  }
}


