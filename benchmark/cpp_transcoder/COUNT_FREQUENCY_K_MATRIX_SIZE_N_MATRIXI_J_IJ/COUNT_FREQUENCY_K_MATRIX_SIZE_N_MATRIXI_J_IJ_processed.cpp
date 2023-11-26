
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n, int k ) {
  if ( n + 1 >= k ) return ( k - 1 );
  else return ( 2 * n + 1 - k );
}


