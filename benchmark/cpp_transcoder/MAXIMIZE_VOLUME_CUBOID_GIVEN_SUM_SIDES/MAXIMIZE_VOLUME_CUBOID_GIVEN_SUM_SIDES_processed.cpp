
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int s ) {
  int maxvalue = 0;
  for ( int i = 1;
  i <= s - 2;
  i ++ ) {
    for ( int j = 1;
    j <= s - 1;
    j ++ ) {
      int k = s - i - j;
      maxvalue = max ( maxvalue, i * j * k );
    }
  }
  return maxvalue;
}


