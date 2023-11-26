
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int x1, int y1, int x2, int y2 ) {
  return ( x1 * ( y2 - y1 ) == y1 * ( x2 - x1 ) );
}


