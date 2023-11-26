
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string N ) {
  int len = N . length ( );
  int l = ( len ) / 2;
  int count = 0;
  for ( int i = 1;
  i <= l;
  i ++ ) {
    string s = N . substr ( 0, i );
    int l1 = s . length ( );
    string t = N . substr ( i, l1 );
    if ( s [ 0 ] == '0' || t [ 0 ] == '0' ) continue;
    if ( s . compare ( t ) == 0 ) count ++;
  }
  return count;
}


