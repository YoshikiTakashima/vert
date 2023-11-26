
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr1 [ ], int arr2 [ ], int n ) {
  for ( int i = 0;
  i < n;
  i ++ ) if ( arr1 [ i ] != arr2 [ i ] ) return i;
  return n;
}


