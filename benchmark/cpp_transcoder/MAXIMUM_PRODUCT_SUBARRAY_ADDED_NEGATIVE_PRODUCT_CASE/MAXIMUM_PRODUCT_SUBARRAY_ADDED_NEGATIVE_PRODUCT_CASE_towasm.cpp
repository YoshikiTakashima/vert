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
  int i;
  int ans = INT_MIN;
  int maxval = 1;
  int minval = 1;
  int prevMax;
  for ( i = 0;
  i < n;
  i ++ ) {
    if ( arr [ i ] > 0 ) {
      maxval = maxval * arr [ i ];
      minval = min ( 1, minval * arr [ i ] );
    }
    else if ( arr [ i ] == 0 ) {
      minval = 1;
      maxval = 0;
    }
    else if ( arr [ i ] < 0 ) {
      prevMax = maxval;
      maxval = minval * arr [ i ];
      minval = prevMax * arr [ i ];
    }
    ans = max ( ans, maxval );
    if ( maxval <= 0 ) {
      maxval = 1;
    }
  }
  return ans;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}