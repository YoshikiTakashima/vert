
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
float f_gold ( int n ) {
  int i = 1;
  double res = 0.0;
  bool sign = 1;
  while ( n > 0 ) {
    n --;
    if ( sign ) {
      sign = ! sign;
      res = res + ( double ) ++ i / ++ i;
    }
    else {
      sign = ! sign;
      res = res - ( double ) ++ i / ++ i;
    }
  }
  return res;
}


