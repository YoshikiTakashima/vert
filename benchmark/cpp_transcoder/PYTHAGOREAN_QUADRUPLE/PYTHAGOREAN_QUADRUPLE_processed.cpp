
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a, int b, int c, int d ) {
  int sum = a * a + b * b + c * c;
  if ( d * d == sum ) return 1;
  else return 0;
}


