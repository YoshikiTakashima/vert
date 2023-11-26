
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  int dp [ n + 1 ] [ 27 ];
  memset ( dp, 0, sizeof ( dp ) );
  for ( int i = 0;
  i <= 25;
  i ++ ) dp [ 1 ] [ i ] = 1;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    for ( int j = 0;
    j <= 25;
    j ++ ) if ( j == 0 ) dp [ i ] [ j ] = dp [ i - 1 ] [ j + 1 ];
    else dp [ i ] [ j ] = ( dp [ i - 1 ] [ j - 1 ] + dp [ i - 1 ] [ j + 1 ] );
  }
  int sum = 0;
  for ( int i = 0;
  i <= 25;
  i ++ ) sum = ( sum + dp [ n ] [ i ] );
  return sum;
}


