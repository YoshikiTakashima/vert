
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int limit ) {
  if ( limit < 2 ) return 0;
  long int ef1 = 0, ef2 = 2;
  long int sum = ef1 + ef2;
  while ( ef2 <= limit ) {
    long int ef3 = 4 * ef2 + ef1;
    if ( ef3 > limit ) break;
    ef1 = ef2;
    ef2 = ef3;
    sum += ef2;
  }
  return sum;
}


