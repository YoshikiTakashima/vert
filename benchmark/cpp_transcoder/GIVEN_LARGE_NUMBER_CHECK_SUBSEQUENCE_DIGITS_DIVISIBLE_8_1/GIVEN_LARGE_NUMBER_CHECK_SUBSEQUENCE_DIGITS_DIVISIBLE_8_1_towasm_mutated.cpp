#include <stdio.h>
        #include <math.h>
        #include <stdlib.h>
        #include <limits.h>
        #include <stdbool.h>
int min(int x, int y) { return (x < y)? x: y; }
        int max(int x, int y) { return (x > y)? x: y; }
        int cmpfunc (const void * a, const void * b) {return ( *(int*)a - *(int*)b );}
        int len (int arr [ ]) {return ((int) (sizeof (arr) / sizeof (arr)[0]));}
        void sort (int arr [ ], int n) {qsort (arr, n, sizeof(int), cmpfunc);}

#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( string str ) {
  int n = str . length ( );
  int dp [ n + 1 ] [ 10 ];
  memset ( dp, 0, sizeof ( dp ) );
  int arr [ n + 1 ];
  for ( int i = 1;
  i <= n;
  i ++ ) arr [ i ] = str [ i - 1 ] - '0';
  for ( int i = 1;
  i <= n;
  i ++ ) {
    dp [ i ] [ arr [ i ] % 8 ] = 1;
    for ( int j = 0;
    j < 8;
    j ++ ) {
      if ( dp [ i - 1 ] [ j ] > dp [ i ] [ ( j * 10 + arr [ i ] ) % 8 ] ) dp [ i ] [ ( j * 10 + arr [ i ] ) % 8 ] = dp [ i - 1 ] [ j ];
      if ( dp [ i - 1 ] [ j ] > dp [ i ] [ j ] ) dp [ i ] [ j ] = dp [ i - 1 ] [ j ];
    }
  }
  for ( int i = 1;
  i <= n;
  i ++ ) {
    if ( dp [ i ] [ 0 ] == 1 ) return 1;
  }
  return 0;
}


int main(void) {
		f_gold("cd");
}