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
  int min_ending_here = INT_MAX;
  int min_so_far = INT_MAX;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( min_ending_here > 0 ) min_ending_here = arr [ i ];
    else min_ending_here += arr [ i ];
    min_so_far = min ( min_so_far, min_ending_here );
  }
  return min_so_far;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}