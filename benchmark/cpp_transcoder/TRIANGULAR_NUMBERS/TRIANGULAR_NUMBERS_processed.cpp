
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int num ) {
  if ( num < 0 ) return 0;
  int sum = 0;
  for ( int n = 1;
  sum <= num;
  n ++ ) {
    sum = sum + n;
    if ( sum == num ) return 1;
  }
  return 0;
}


