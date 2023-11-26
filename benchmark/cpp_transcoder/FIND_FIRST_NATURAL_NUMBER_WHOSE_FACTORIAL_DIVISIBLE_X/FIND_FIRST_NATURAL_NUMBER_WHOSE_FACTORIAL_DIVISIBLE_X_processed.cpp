
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int x ) {
  int i = 1;
  int fact = 1;
  for ( i = 1;
  i < x;
  i ++ ) {
    fact = fact * i;
    if ( fact % x == 0 ) break;
  }
  return i;
}


