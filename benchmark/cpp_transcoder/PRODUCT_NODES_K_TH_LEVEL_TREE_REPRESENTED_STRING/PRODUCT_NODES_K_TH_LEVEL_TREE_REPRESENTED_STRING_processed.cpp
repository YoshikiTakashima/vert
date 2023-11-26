
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string tree, int k ) {
  int level = - 1;
  int product = 1;
  int n = tree . length ( );
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( tree [ i ] == '(' ) level ++;
    else if ( tree [ i ] == ')' ) level --;
    else {
      if ( level == k ) product *= ( tree [ i ] - '0' );
    }
  }
  return product;
}


