
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int der [ n + 1 ];
  der [ 0 ] = 1;
  der [ 1 ] = 0;
  der [ 2 ] = 1;
  for ( int i = 3;
  i <= n;
  ++ i ) der [ i ] = ( i - 1 ) * ( der [ i - 1 ] + der [ i - 2 ] );
  return der [ n ];
}


