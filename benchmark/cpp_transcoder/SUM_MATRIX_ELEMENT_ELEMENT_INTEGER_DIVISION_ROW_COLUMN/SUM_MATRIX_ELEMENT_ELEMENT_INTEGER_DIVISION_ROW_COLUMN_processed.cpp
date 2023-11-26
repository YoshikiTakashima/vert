
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int ans = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) for ( int j = 1;
  j <= n;
  j ++ ) ans += ( i / j );
  return ans;
}


