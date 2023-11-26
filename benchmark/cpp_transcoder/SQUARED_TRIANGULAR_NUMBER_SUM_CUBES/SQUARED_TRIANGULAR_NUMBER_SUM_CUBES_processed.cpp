
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int s ) {
  int sum = 0;
  for ( int n = 1;
  sum < s;
  n ++ ) {
    sum += n * n * n;
    if ( sum == s ) return n;
  }
  return - 1;
}


