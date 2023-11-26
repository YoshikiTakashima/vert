
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int res = 0;
  for ( int x = 0;
  x * x < n;
  x ++ ) for ( int y = 0;
  x * x + y * y < n;
  y ++ ) res ++;
  return res;
}


