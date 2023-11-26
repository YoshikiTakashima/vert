
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( char str [ ], int n ) {
  char last = ' ';
  int res = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( last != str [ i ] ) res ++;
    last = str [ i ];
  }
  return res / 2;
}


