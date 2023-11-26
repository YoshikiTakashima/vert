
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int i = 0;
  int ans = 0;
  while ( ( 1 << i ) <= n ) {
    bool k = 0;
    int change = 1 << i;
    for ( int j = 0;
    j <= n;
    j ++ ) {
      ans += k;
      if ( change == 1 ) {
        k = ! k;
        change = 1 << i;
      }
      else {
        change --;
      }
    }
    i ++;
  }
  return ans;
}


