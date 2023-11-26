
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  long int sum = 0;
  sum = 1 << n;
  return ( sum - 1 );
}


