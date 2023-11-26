
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( long n ) {
  int S = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) S += i * i - ( i - 1 ) * ( i - 1 );
  return S;
}


