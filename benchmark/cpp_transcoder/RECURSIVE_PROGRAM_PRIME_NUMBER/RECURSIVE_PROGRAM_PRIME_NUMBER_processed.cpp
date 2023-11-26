
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n, int i = 2 ) {
  if ( n <= 2 ) return ( n == 2 ) ? 1 : 0;
  if ( n % i == 0 ) return 0;
  if ( i * i > n ) return 1;
  return f_gold ( n, i + 1 );
}


