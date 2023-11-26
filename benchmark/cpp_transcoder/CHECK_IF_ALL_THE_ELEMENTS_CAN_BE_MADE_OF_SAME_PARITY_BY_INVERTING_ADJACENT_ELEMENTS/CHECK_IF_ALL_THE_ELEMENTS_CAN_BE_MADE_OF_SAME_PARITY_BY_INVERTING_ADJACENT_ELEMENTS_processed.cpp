
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a [ ], int n ) {
  int count_odd = 0, count_even = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( a [ i ] & 1 ) count_odd ++;
    else count_even ++;
  }
  if ( count_odd % 2 && count_even % 2 ) return 0;
  else return 1;
}


