
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int x, int y ) {
  int res = 1;
  for ( int i = 0;
  i < y;
  i ++ ) res = ( res * x ) % 10;
  return res;
}


