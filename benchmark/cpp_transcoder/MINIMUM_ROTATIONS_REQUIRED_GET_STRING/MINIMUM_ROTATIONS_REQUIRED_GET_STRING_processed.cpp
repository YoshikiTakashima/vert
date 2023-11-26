
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  string tmp = str + str;
  int n = str . length ( );
  for ( int i = 1;
  i <= n;
  i ++ ) {
    string substring = tmp . substr ( i, str . size ( ) );
    if ( str == substring ) return i;
  }
  return n;
}


