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
  int dp [ n ];
  if ( n == 1 ) return arr [ 0 ];
  if ( n == 2 ) return min ( arr [ 0 ], arr [ 1 ] );
  if ( n == 3 ) return min ( arr [ 0 ], min ( arr [ 1 ], arr [ 2 ] ) );
  if ( n == 4 ) return min ( min ( arr [ 0 ], arr [ 1 ] ), min ( arr [ 2 ], arr [ 3 ] ) );
  dp [ 0 ] = arr [ 0 ];
  dp [ 1 ] = arr [ 1 ];
  dp [ 2 ] = arr [ 2 ];
  dp [ 3 ] = arr [ 3 ];
  for ( int i = 4;
  i < n;
  i ++ ) dp [ i ] = arr [ i ] + min ( min ( dp [ i - 1 ], dp [ i - 2 ] ), min ( dp [ i - 3 ], dp [ i - 4 ] ) );
  return min ( min ( dp [ n - 1 ], dp [ n - 2 ] ), min ( dp [ n - 4 ], dp [ n - 3 ] ) );
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}