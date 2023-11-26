
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int A [ ], int n ) {
  int min_val = * min_element ( A, A + n );
  return ( min_val * ( n - 1 ) );
}


