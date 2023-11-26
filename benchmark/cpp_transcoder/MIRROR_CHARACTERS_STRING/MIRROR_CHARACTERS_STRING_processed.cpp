
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
string f_gold ( string str, int n ) {
  string reverseAlphabet = "zyxwvutsrqponmlkjihgfedcba";
  int l = str . length ( );
  for ( int i = n;
  i < l;
  i ++ ) str [ i ] = reverseAlphabet [ str [ i ] - 'a' ];
  return str;
}


