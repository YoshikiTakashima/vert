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
  int leftMax [ n ];
  leftMax [ 0 ] = INT_MIN;
  for ( int i = 1;
  i < n;
  i ++ ) leftMax [ i ] = max ( leftMax [ i - 1 ], arr [ i - 1 ] );
  int rightMin = INT_MAX;
  for ( int i = n - 1;
  i >= 0;
  i -- ) {
    if ( leftMax [ i ] < arr [ i ] && rightMin > arr [ i ] ) return i;
    rightMin = min ( rightMin, arr [ i ] );
  }
  return - 1;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}