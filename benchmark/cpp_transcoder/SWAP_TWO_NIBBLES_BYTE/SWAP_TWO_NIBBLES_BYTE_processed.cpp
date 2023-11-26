
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int x ) {
  return ( ( x & 0x0F ) << 4 | ( x & 0xF0 ) >> 4 );
}


