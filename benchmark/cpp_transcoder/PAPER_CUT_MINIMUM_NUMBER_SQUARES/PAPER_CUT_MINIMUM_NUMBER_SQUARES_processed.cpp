
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a, int b ) {
  long long result = 0, rem = 0;
  if ( a < b ) swap ( a, b );
  while ( b > 0 ) {
    result += a / b;
    long long rem = a % b;
    a = b;
    b = rem;
  }
  return result;
}


