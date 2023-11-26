
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int ar [ ], int ar_size ) {
  int res = 0;
  for ( int i = 0;
  i < ar_size;
  i ++ ) res = res ^ ar [ i ];
  return res;
}


