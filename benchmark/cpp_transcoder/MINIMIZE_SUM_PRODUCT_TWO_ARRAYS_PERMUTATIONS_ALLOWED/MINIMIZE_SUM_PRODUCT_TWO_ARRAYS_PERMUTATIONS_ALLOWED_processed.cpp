
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int A [ ], int B [ ], int n ) {
  sort ( A, A + n );
  sort ( B, B + n );
  int result = 0;
  for ( int i = 0;
  i < n;
  i ++ ) result += ( A [ i ] * B [ n - i - 1 ] );
  return result;
}


