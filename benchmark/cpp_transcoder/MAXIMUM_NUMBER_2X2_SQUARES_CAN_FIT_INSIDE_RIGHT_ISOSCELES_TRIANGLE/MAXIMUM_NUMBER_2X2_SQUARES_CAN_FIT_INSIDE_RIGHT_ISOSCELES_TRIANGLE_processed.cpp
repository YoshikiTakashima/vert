
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int base ) {
  base = ( base - 2 );
  base = base / 2;
  return base * ( base + 1 ) / 2;
}


