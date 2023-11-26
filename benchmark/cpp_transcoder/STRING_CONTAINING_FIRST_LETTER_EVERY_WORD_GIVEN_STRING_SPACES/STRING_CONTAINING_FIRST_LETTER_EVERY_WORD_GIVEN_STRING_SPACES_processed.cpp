
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
string f_gold ( string str ) {
  string result = "";
  bool v = 1;
  for ( int i = 0;
  i < str . length ( );
  i ++ ) {
    if ( str [ i ] == ' ' ) v = 1;
    else if ( str [ i ] != ' ' && v == 1 ) {
      result . push_back ( str [ i ] );
      v = 0;
    }
  }
  return result;
}


