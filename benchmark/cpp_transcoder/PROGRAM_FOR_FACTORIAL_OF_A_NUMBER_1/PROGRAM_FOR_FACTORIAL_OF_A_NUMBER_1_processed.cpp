
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
unsigned int f_gold ( unsigned int n ) {
  int res = 1, i;
  for ( i = 2;
  i <= n;
  i ++ ) res *= i;
  return res;
}


