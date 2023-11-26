
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  long int sum = 0;
  for ( int row = 0;
  row < n;
  row ++ ) {
    sum = sum + ( 1 << row );
  }
  return sum;
}


