
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  int res = 1;
  for ( int i = 0;
  i < n && arr [ i ] <= res;
  i ++ ) res = res + arr [ i ];
  return res;
}


