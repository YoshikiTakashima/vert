
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int num ) {
  if ( num / 10 == 0 ) return 1;
  while ( num != 0 ) {
    if ( num / 10 == 0 ) return 1;
    int digit1 = num % 10;
    int digit2 = ( num / 10 ) % 10;
    if ( abs ( digit2 - digit1 ) > 1 ) return 0;
    num = num / 10;
  }
  return 1;
}


