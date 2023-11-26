

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

int f_gold ( int arr [ ], int N, int K ) {
  sort ( arr, arr + N );
  int res = INT_MAX;
  for ( int i = 0;
  i <= ( N - K );
  i ++ ) {
    int curSeqDiff = arr [ i + K - 1 ] - arr [ i ];
    res = min ( res, curSeqDiff );
  }
  return res;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,29);
}