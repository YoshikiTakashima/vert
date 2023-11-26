
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n, int k, int a [ ] ) {
  sort ( a, a + n, greater < int > ( ) );
  int f_gold = 0;
  for ( int i = 0;
  i < n;
  i += k ) f_gold += ( 2 * a [ i ] );
  return f_gold;
}


