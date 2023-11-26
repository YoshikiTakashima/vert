
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int a = 1, b = 2, c = 0;
  if ( n <= 2 ) {
    return n;
  }
  for ( int i = 3;
  i <= n;
  i ++ ) {
    c = b + ( i - 1 ) * a;
    a = b;
    b = c;
  }
  return c;
}


