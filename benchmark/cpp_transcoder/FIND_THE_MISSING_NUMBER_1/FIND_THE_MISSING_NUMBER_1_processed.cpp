
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a [ ], int n ) {
  int i, total = 1;
  for ( i = 2;
  i <= ( n + 1 );
  i ++ ) {
    total += i;
    total -= a [ i - 2 ];
  }
  return total;
}


