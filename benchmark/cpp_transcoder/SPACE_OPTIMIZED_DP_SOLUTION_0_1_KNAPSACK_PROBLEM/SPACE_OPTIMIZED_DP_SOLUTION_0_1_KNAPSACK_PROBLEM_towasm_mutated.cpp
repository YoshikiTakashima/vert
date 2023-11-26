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
int f_gold ( int val [ ], int wt [ ], int n, int W ) {
  int mat [ 2 ] [ W + 1 ];
  memset ( mat, 0, sizeof ( mat ) );
  int i = 0;
  while ( i < n ) {
    int j = 0;
    if ( i % 2 != 0 ) {
      while ( ++ j <= W ) {
        if ( wt [ i ] <= j ) mat [ 1 ] [ j ] = max ( val [ i ] + mat [ 0 ] [ j - wt [ i ] ], mat [ 0 ] [ j ] );
        else mat [ 1 ] [ j ] = mat [ 0 ] [ j ];
      }
    }
    else {
      while ( ++ j <= W ) {
        if ( wt [ i ] <= j ) mat [ 0 ] [ j ] = max ( val [ i ] + mat [ 1 ] [ j - wt [ i ] ], mat [ 1 ] [ j ] );
        else mat [ 0 ] [ j ] = mat [ 1 ] [ j ];
      }
    }
    i ++;
  }
  return ( n % 2 != 0 ) ? mat [ 0 ] [ W ] : mat [ 1 ] [ W ];
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3,29);
}