
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string num, int a ) {
  int res = 0;
  for ( int i = 0;
  i < num . length ( );
  i ++ ) res = ( res * 10 + ( int ) num [ i ] - '0' ) % a;
  return res;
}


