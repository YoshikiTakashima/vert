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
  int min_prefix_sum = 0;
  int res = numeric_limits < int > :: min ( );
  int prefix_sum [ n ];
  prefix_sum [ 0 ] = arr [ 0 ];
  for ( int i = 1;
  i < n;
  i ++ ) prefix_sum [ i ] = prefix_sum [ i - 1 ] + arr [ i ];
  for ( int i = 0;
  i < n;
  i ++ ) {
    res = max ( res, prefix_sum [ i ] - min_prefix_sum );
    min_prefix_sum = min ( min_prefix_sum, prefix_sum [ i ] );
  }
  return res;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}