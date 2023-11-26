
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int sum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) sum += i * ( n - i );
  return 2 * sum;
}


