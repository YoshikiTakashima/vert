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
int f_gold ( int W, int n, int val [ ], int wt [ ] ) {
  int dp [ W + 1 ];
  memset ( dp, 0, sizeof dp );
  int ans = 0;
  for ( int i = 0;
  i <= W;
  i ++ ) for ( int j = 0;
  j < n;
  j ++ ) if ( wt [ j ] <= i ) dp [ i ] = max ( dp [ i ], dp [ i - wt [ j ] ] + val [ j ] );
  return dp [ W ];
}


int main(void) {
		int qe[] = {11,12};
	int rp[] = {11,12};
	f_gold(1,2,qe,rp);
}