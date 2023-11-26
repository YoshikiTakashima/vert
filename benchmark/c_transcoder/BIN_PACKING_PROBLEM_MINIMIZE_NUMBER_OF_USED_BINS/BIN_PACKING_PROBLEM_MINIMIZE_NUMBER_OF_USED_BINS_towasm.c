

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

int f_gold ( int weight [ ], int n, int c ) {
  int res = 0, bin_rem = c;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( weight [ i ] > bin_rem ) {
      res ++;
      bin_rem = c - weight [ i ];
    }
    else bin_rem -= weight [ i ];
  }
  return res;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}