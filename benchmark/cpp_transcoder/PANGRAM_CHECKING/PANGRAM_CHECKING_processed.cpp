
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  vector < bool > mark ( 26, 0 );
  int index;
  for ( int i = 0;
  i < str . length ( );
  i ++ ) {
    if ( 'A' <= str [ i ] && str [ i ] <= 'Z' ) index = str [ i ] - 'A';
    else if ( 'a' <= str [ i ] && str [ i ] <= 'z' ) index = str [ i ] - 'a';
    mark [ index ] = 1;
  }
  for ( int i = 0;
  i <= 25;
  i ++ ) if ( mark [ i ] == 0 ) return ( 0 );
  return ( 1 );
}


