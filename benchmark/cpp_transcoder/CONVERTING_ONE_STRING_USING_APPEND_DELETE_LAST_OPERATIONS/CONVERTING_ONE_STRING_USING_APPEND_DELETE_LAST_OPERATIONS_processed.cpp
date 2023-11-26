
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str1, string str2, int k ) {
  if ( ( str1 . length ( ) + str2 . length ( ) ) < k ) return 1;
  int commonLength = 0;
  for ( int i = 0;
  i < min ( str1 . length ( ), str2 . length ( ) );
  i ++ ) {
    if ( str1 [ i ] == str2 [ i ] ) commonLength ++;
    else break;
  }
  if ( ( k - str1 . length ( ) - str2 . length ( ) + 2 * commonLength ) % 2 == 0 ) return 1;
  return 0;
}


