
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
float f_gold ( double a, double b ) {
  double mod;
  if ( a < 0 ) mod = - a;
  else mod = a;
  if ( b < 0 ) b = - b;
  while ( mod >= b ) mod = mod - b;
  if ( a < 0 ) return - mod;
  return mod;
}


