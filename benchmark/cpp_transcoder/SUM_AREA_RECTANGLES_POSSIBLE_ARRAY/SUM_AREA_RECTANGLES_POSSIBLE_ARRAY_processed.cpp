
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a [ ], int n ) {
  sort ( a, a + n, greater < int > ( ) );
  int sum = 0;
  bool flag = 0;
  int len;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( ( a [ i ] == a [ i + 1 ] || a [ i ] - a [ i + 1 ] == 1 ) && ( ! flag ) ) {
      flag = 1;
      len = a [ i + 1 ];
      i ++;
    }
    else if ( ( a [ i ] == a [ i + 1 ] || a [ i ] - a [ i + 1 ] == 1 ) && ( flag ) ) {
      sum = sum + a [ i + 1 ] * len;
      flag = 0;
      i ++;
    }
  }
  return sum;
}


