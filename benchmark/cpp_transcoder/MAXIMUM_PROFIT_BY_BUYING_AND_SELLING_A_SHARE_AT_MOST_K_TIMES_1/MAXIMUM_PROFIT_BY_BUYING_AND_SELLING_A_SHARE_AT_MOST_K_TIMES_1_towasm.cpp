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
int f_gold ( int price [ ], int n, int k ) {
  int profit [ k + 1 ] [ n + 1 ];
  for ( int i = 0;
  i <= k;
  i ++ ) profit [ i ] [ 0 ] = 0;
  for ( int j = 0;
  j <= n;
  j ++ ) profit [ 0 ] [ j ] = 0;
  for ( int i = 1;
  i <= k;
  i ++ ) {
    int prevDiff = INT_MIN;
    for ( int j = 1;
    j < n;
    j ++ ) {
      prevDiff = max ( prevDiff, profit [ i - 1 ] [ j - 1 ] - price [ j - 1 ] );
      profit [ i ] [ j ] = max ( profit [ i ] [ j - 1 ], price [ j ] + prevDiff );
    }
  }
  return profit [ k ] [ n - 1 ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}