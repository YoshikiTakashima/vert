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
int f_gold ( int arr [ ], int N, int K ) {
  sort ( arr, arr + N );
  int dp [ N ];
  dp [ 0 ] = 0;
  for ( int i = 1;
  i < N;
  i ++ ) {
    dp [ i ] = dp [ i - 1 ];
    if ( arr [ i ] - arr [ i - 1 ] < K ) {
      if ( i >= 2 ) dp [ i ] = max ( dp [ i ], dp [ i - 2 ] + arr [ i ] + arr [ i - 1 ] );
      else dp [ i ] = max ( dp [ i ], arr [ i ] + arr [ i - 1 ] );
    }
  }
  return dp [ N - 1 ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}