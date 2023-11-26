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
  int sum [ n ];
  if ( n >= 1 ) sum [ 0 ] = arr [ 0 ];
  if ( n >= 2 ) sum [ 1 ] = arr [ 0 ] + arr [ 1 ];
  if ( n > 2 ) sum [ 2 ] = max ( sum [ 1 ], max ( arr [ 1 ] + arr [ 2 ], arr [ 0 ] + arr [ 2 ] ) );
  for ( int i = 3;
  i < n;
  i ++ ) sum [ i ] = max ( max ( sum [ i - 1 ], sum [ i - 2 ] + arr [ i ] ), arr [ i ] + arr [ i - 1 ] + sum [ i - 3 ] );
  return sum [ n - 1 ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}