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
int f_gold ( int arr [ ], int n, int m ) {
  if ( m == 0 || n == 0 ) return 0;
  sort ( arr, arr + n );
  if ( n < m ) return - 1;
  int min_diff = INT_MAX;
  int first = 0, last = 0;
  for ( int i = 0;
  i + m - 1 < n;
  i ++ ) {
    int diff = arr [ i + m - 1 ] - arr [ i ];
    if ( diff < min_diff ) {
      min_diff = diff;
      first = i;
      last = i + m - 1;
    }
  }
  return ( arr [ last ] - arr [ first ] );
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}