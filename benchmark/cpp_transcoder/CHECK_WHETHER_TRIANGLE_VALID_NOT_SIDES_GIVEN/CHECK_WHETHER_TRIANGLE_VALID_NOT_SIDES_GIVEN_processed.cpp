
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a, int b, int c ) {
  if ( a + b <= c || a + c <= b || b + c <= a ) return 0;
  else return 1;
}


