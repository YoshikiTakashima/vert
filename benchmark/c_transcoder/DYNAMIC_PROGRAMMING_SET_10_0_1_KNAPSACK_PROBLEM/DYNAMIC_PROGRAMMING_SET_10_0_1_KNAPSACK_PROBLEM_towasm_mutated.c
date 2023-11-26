

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

int f_gold ( int W, int wt [ ], int val [ ], int n ) {
  if ( n == 0 || W == 0 ) return 0;
  if ( wt [ n - 1 ] > W ) return f_gold ( W, wt, val, n - 1 );
  else return max ( val [ n - 1 ] + f_gold ( W - wt [ n - 1 ], wt, val, n - 1 ), f_gold ( W, wt, val, n - 1 ) );
}


int main(void) {
		int yq[] = {11,12};
	int qe[] = {11,12};
	f_gold(1,yq,qe,29);
}