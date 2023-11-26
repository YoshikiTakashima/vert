
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
char f_gold ( string strA, string strB ) {
  int res = 0, i;
  for ( i = 0;
  i < strA . length ( );
  i ++ ) {
    res ^= strA [ i ];
  }
  for ( i = 0;
  i < strB . length ( );
  i ++ ) {
    res ^= strB [ i ];
  }
  return ( ( char ) ( res ) );
}


