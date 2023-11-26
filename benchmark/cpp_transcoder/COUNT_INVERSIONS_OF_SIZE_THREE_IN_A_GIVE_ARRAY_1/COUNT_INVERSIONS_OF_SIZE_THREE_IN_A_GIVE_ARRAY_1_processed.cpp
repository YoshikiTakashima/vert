
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  int invcount = 0;
  for ( int i = 1;
  i < n - 1;
  i ++ ) {
    int small = 0;
    for ( int j = i + 1;
    j < n;
    j ++ ) if ( arr [ i ] > arr [ j ] ) small ++;
    int great = 0;
    for ( int j = i - 1;
    j >= 0;
    j -- ) if ( arr [ i ] < arr [ j ] ) great ++;
    invcount += great * small;
  }
  return invcount;
}


