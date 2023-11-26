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
int f_gold ( char s [ ], int n ) {
  int dp [ n ] [ n ];
  memset ( dp, 0, sizeof ( dp ) );
  for ( int i = 0;
  i < n - 1;
  i ++ ) if ( s [ i ] == '(' && s [ i + 1 ] == ')' ) dp [ i ] [ i + 1 ] = 2;
  for ( int l = 2;
  l < n;
  l ++ ) {
    for ( int i = 0, j = l;
    j < n;
    i ++, j ++ ) {
      if ( s [ i ] == '(' && s [ j ] == ')' ) dp [ i ] [ j ] = 2 + dp [ i + 1 ] [ j - 1 ];
      for ( int k = i;
      k < j;
      k ++ ) dp [ i ] [ j ] = max ( dp [ i ] [ j ], dp [ i ] [ k ] + dp [ k + 1 ] [ j ] );
    }
  }
  return dp [ 0 ] [ n - 1 ];
}


int main(void) {
	char xv[] = {'a','b'};
	f_gold(xv,2);
}