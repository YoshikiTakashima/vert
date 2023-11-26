
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  if ( n == 0 ) return 0;
  else return 1 + f_gold ( n & ( n - 1 ) );
}


