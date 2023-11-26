
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n, int k ) {
  if ( k <= 0 ) return n;
  return ( n & ~ ( 1 << ( k - 1 ) ) );
}


