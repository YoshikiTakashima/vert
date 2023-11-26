
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a, int b, int c ) {
  if ( ( a < b && b < c ) || ( c < b && b < a ) ) return b;
  else if ( ( b < a && a < c ) || ( c < a && a < b ) ) return a;
  else return c;
}


