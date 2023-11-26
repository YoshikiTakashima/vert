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
int f_gold ( string a, string b ) {
  int n = a . size ( ), m = b . size ( );
  if ( m == 0 ) return 1;
  int dp [ m + 1 ] [ n + 1 ];
  memset ( dp, 0, sizeof ( dp ) );
  for ( int i = 0;
  i < m;
  i ++ ) {
    for ( int j = i;
    j < n;
    j ++ ) {
      if ( i == 0 ) {
        if ( j == 0 ) dp [ i ] [ j ] = ( a [ j ] == b [ i ] ) ? 1 : 0;
        else if ( a [ j ] == b [ i ] ) dp [ i ] [ j ] = dp [ i ] [ j - 1 ] + 1;
        else dp [ i ] [ j ] = dp [ i ] [ j - 1 ];
      }
      else {
        if ( a [ j ] == b [ i ] ) dp [ i ] [ j ] = dp [ i ] [ j - 1 ] + dp [ i - 1 ] [ j - 1 ];
        else dp [ i ] [ j ] = dp [ i ] [ j - 1 ];
      }
    }
  }
  return dp [ m - 1 ] [ n - 1 ];
}


int main(void) {
		f_gold("cd","cb");
}