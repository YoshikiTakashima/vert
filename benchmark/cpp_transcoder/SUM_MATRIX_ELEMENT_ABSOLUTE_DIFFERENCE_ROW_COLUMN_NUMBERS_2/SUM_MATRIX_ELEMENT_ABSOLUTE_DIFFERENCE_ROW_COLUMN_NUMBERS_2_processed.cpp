
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  n --;
  int sum = 0;
  sum += ( n * ( n + 1 ) ) / 2;
  sum += ( n * ( n + 1 ) * ( 2 * n + 1 ) ) / 6;
  return sum;
}


