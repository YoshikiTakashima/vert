
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int num, int divisor ) {
  return ( num - divisor * ( num / divisor ) );
}


