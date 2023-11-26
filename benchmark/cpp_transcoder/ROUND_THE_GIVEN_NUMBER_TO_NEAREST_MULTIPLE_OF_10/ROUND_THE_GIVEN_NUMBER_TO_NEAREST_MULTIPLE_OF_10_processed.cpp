
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int a = ( n / 10 ) * 10;
  int b = a + 10;
  return ( n - a > b - n ) ? b : a;
}


