
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int num ) {
  if ( num < 0 ) return f_gold ( - num );
  if ( num == 0 || num == 7 ) return 1;
  if ( num < 10 ) return 0;
  return f_gold ( num / 10 - 2 * ( num - num / 10 * 10 ) );
}


