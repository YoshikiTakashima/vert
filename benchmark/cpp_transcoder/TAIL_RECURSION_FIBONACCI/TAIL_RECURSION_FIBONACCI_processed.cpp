
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n, int a = 0, int b = 1 ) {
  if ( n == 0 ) return a;
  if ( n == 1 ) return b;
  return f_gold ( n - 1, b, a + b );
}


