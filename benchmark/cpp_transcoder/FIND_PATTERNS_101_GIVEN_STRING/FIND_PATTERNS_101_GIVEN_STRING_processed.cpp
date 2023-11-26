
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  char last = str [ 0 ];
  int i = 1, counter = 0;
  while ( i < str . size ( ) ) {
    if ( str [ i ] == '0' && last == '1' ) {
      while ( str [ i ] == '0' ) i ++;
      if ( str [ i ] == '1' ) counter ++;
    }
    last = str [ i ];
    i ++;
  }
  return counter;
}


