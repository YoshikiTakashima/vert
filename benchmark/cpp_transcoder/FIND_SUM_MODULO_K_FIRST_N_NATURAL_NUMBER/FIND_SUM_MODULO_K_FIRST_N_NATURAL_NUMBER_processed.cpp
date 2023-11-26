
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int N, int K ) {
  int ans = 0;
  for ( int i = 1;
  i <= N;
  i ++ ) ans += ( i % K );
  return ans;
}


