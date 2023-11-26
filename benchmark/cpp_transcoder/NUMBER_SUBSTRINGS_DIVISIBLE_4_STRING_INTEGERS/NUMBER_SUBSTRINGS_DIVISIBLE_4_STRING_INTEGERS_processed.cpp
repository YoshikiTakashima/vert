
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( char s [ ] ) {
  int n = strlen ( s );
  int count = 0;
  for ( int i = 0;
  i < n;
  ++ i ) if ( s [ i ] == '4' || s [ i ] == '8' || s [ i ] == '0' ) count ++;
  for ( int i = 0;
  i < n - 1;
  ++ i ) {
    int h = ( s [ i ] - '0' ) * 10 + ( s [ i + 1 ] - '0' );
    if ( h % 4 == 0 ) count = count + i + 1;
  }
  return count;
}


