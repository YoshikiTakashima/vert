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
  int i, j, max = 0;
  int msis [ n ];
  for ( i = 0;
  i < n;
  i ++ ) msis [ i ] = arr [ i ];
  for ( i = 1;
  i < n;
  i ++ ) for ( j = 0;
  j < i;
  j ++ ) if ( arr [ i ] > arr [ j ] && msis [ i ] < msis [ j ] + arr [ i ] ) msis [ i ] = msis [ j ] + arr [ i ];
  for ( i = 0;
  i < n;
  i ++ ) if ( max < msis [ i ] ) max = msis [ i ];
  return max;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}