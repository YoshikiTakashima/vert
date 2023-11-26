
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  for ( int sum = 0, i = 1;
  sum < n;
  i += 2 ) {
    sum += i;
    if ( sum == n ) return 1;
  }
  return 0;
}


