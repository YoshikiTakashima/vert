
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string n ) {
  int i = 0, pos = 0;
  while ( n [ i ] != '\0' ) {
    switch ( n [ i ] ) {
      case '4' : pos = pos * 2 + 1;
      break;
      case '7' : pos = pos * 2 + 2;
      break;
    }
    i ++;
  }
  return pos;
}


