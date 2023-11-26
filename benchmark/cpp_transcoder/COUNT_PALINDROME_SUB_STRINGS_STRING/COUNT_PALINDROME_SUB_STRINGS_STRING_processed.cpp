
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( char str [ ], int n ) {
  int dp [ n ] [ n ];
  memset ( dp, 0, sizeof ( dp ) );
  bool P [ n ] [ n ];
  memset ( P, 0, sizeof ( P ) );
  for ( int i = 0;
  i < n;
  i ++ ) P [ i ] [ i ] = 1;
  for ( int i = 0;
  i < n - 1;
  i ++ ) {
    if ( str [ i ] == str [ i + 1 ] ) {
      P [ i ] [ i + 1 ] = 1;
      dp [ i ] [ i + 1 ] = 1;
    }
  }
  for ( int gap = 2;
  gap < n;
  gap ++ ) {
    for ( int i = 0;
    i < n - gap;
    i ++ ) {
      int j = gap + i;
      if ( str [ i ] == str [ j ] && P [ i + 1 ] [ j - 1 ] ) P [ i ] [ j ] = 1;
      if ( P [ i ] [ j ] == 1 ) dp [ i ] [ j ] = dp [ i ] [ j - 1 ] + dp [ i + 1 ] [ j ] + 1 - dp [ i + 1 ] [ j - 1 ];
      else dp [ i ] [ j ] = dp [ i ] [ j - 1 ] + dp [ i + 1 ] [ j ] - dp [ i + 1 ] [ j - 1 ];
    }
  }
  return dp [ 0 ] [ n - 1 ];
}


