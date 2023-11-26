
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  string temp = "";
  int sum = 0;
  for ( char ch : str ) {
    if ( isdigit ( ch ) ) temp += ch;
    else {
      sum += atoi ( temp . c_str ( ) );
      temp = "";
    }
  }
  return sum + atoi ( temp . c_str ( ) );
}


