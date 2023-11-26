
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  int i = 0, j = str . length ( ) - 1;
  while ( i < j ) {
    if ( str [ i ] != str [ j ] ) return 0;
    i ++;
    j --;
  }
  return 1;
}


