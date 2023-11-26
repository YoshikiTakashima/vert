
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str, string corner ) {
  int n = str . length ( );
  int cl = corner . length ( );
  if ( n < cl ) return 0;
  return ( str . substr ( 0, cl ) . compare ( corner ) == 0 && str . substr ( n - cl, cl ) . compare ( corner ) == 0 );
}


