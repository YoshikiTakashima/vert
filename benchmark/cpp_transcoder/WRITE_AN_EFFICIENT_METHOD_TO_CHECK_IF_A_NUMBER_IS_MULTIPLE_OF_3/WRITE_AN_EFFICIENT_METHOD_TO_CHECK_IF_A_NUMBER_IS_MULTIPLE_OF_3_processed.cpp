
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int odd_count = 0;
  int even_count = 0;
  if ( n < 0 ) n = - n;
  if ( n == 0 ) return 1;
  if ( n == 1 ) return 0;
  while ( n ) {
    if ( n & 1 ) odd_count ++;
    if ( n & 2 ) even_count ++;
    n = n >> 2;
  }
  return f_gold ( abs ( odd_count - even_count ) );
}


