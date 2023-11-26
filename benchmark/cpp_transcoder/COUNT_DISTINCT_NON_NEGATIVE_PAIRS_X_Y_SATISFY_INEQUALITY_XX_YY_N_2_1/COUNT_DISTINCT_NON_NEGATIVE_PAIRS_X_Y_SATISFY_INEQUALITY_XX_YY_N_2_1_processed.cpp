
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int x = 0, yCount, res = 0;
  for ( yCount = 0;
  yCount * yCount < n;
  yCount ++ );
  while ( yCount != 0 ) {
    res += yCount;
    x ++;
    while ( yCount != 0 && ( x * x + ( yCount - 1 ) * ( yCount - 1 ) >= n ) ) yCount --;
  }
  return res;
}


