
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n, int x, int k ) {
  int i = 0;
  while ( i < n ) {
    if ( arr [ i ] == x ) return i;
    i = i + max ( 1, abs ( arr [ i ] - x ) / k );
  }
  cout << "number is not present!";
  return - 1;
}


