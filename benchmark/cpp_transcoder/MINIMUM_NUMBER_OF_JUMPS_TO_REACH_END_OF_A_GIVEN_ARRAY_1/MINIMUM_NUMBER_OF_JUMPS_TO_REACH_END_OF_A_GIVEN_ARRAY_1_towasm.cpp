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
  int * jumps = new int [ n ];
  int i, j;
  if ( n == 0 || arr [ 0 ] == 0 ) return INT_MAX;
  jumps [ 0 ] = 0;
  for ( i = 1;
  i < n;
  i ++ ) {
    jumps [ i ] = INT_MAX;
    for ( j = 0;
    j < i;
    j ++ ) {
      if ( i <= j + arr [ j ] && jumps [ j ] != INT_MAX ) {
        jumps [ i ] = min ( jumps [ i ], jumps [ j ] + 1 );
        break;
      }
    }
  }
  return jumps [ n - 1 ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}