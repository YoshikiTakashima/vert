
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n, int key ) {
  int i;
  for ( i = 0;
  i < n;
  i ++ ) if ( arr [ i ] == key ) return i;
  return - 1;
}


