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
int f_gold ( int p [ ], int n ) {
  int m [ n ] [ n ];
  int i, j, k, L, q;
  for ( i = 1;
  i < n;
  i ++ ) m [ i ] [ i ] = 0;
  for ( L = 2;
  L < n;
  L ++ ) {
    for ( i = 1;
    i < n - L + 1;
    i ++ ) {
      j = i + L - 1;
      m [ i ] [ j ] = INT_MAX;
      for ( k = i;
      k <= j - 1;
      k ++ ) {
        q = m [ i ] [ k ] + m [ k + 1 ] [ j ] + p [ i - 1 ] * p [ k ] * p [ j ];
        if ( q < m [ i ] [ j ] ) m [ i ] [ j ] = q;
      }
    }
  }
  return m [ 1 ] [ n - 1 ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}