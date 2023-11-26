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
int f_gold ( int val [ ], int wt [ ], int n, int W ) {
  int dp [ W + 1 ];
  memset ( dp, 0, sizeof ( dp ) );
  for ( int i = 0;
  i < n;
  i ++ ) for ( int j = W;
  j >= wt [ i ];
  j -- ) dp [ j ] = max ( dp [ j ], val [ i ] + dp [ j - wt [ i ] ] );
  return dp [ W ];
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3,4);
}