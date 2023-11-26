
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int num, int divisor ) {
  while ( num >= divisor ) num -= divisor;
  return num;
}


