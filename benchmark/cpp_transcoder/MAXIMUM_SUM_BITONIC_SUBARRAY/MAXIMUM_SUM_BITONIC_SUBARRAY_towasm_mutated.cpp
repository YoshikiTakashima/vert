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
int f_gold ( int arr [ ], int n ) {
  int msis [ n ], msds [ n ];
  int max_sum = INT_MIN;
  msis [ 0 ] = arr [ 0 ];
  for ( int i = 1;
  i < n;
  i ++ ) if ( arr [ i ] > arr [ i - 1 ] ) msis [ i ] = msis [ i - 1 ] + arr [ i ];
  else msis [ i ] = arr [ i ];
  msds [ n - 1 ] = arr [ n - 1 ];
  for ( int i = n - 2;
  i >= 0;
  i -- ) if ( arr [ i ] > arr [ i + 1 ] ) msds [ i ] = msds [ i + 1 ] + arr [ i ];
  else msds [ i ] = arr [ i ];
  for ( int i = 0;
  i < n;
  i ++ ) if ( max_sum < ( msis [ i ] + msds [ i ] - arr [ i ] ) ) max_sum = msis [ i ] + msds [ i ] - arr [ i ];
  return max_sum;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}