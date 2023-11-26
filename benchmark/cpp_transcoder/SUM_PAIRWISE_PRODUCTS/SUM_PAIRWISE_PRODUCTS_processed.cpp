
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  long int sum = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) for ( int j = i;
  j <= n;
  j ++ ) sum = sum + i * j;
  return sum;
}


