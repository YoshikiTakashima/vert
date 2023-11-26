
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int BT [ n + 1 ];
  memset ( BT, 0, sizeof ( BT ) );
  BT [ 0 ] = BT [ 1 ] = 1;
  for ( int i = 2;
  i <= n;
  ++ i ) for ( int j = 0;
  j < i;
  j ++ ) BT [ i ] += BT [ j ] * BT [ i - j - 1 ];
  return BT [ n ];
}


