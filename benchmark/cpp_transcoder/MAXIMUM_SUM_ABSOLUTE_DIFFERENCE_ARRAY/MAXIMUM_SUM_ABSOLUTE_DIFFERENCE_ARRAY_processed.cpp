
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a [ ], int n ) {
  vector < int > finalSequence;
  sort ( a, a + n );
  for ( int i = 0;
  i < n / 2;
  ++ i ) {
    finalSequence . push_back ( a [ i ] );
    finalSequence . push_back ( a [ n - i - 1 ] );
  }
  int MaximumSum = 0;
  for ( int i = 0;
  i < n - 1;
  ++ i ) {
    MaximumSum = MaximumSum + abs ( finalSequence [ i ] - finalSequence [ i + 1 ] );
  }
  MaximumSum = MaximumSum + abs ( finalSequence [ n - 1 ] - finalSequence [ 0 ] );
  return MaximumSum;
}


