

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

int f_gold ( int x1, int y1, int x2, int y2, int r1, int r2 ) {
  int distSq = ( x1 - x2 ) * ( x1 - x2 ) + ( y1 - y2 ) * ( y1 - y2 );
  int radSumSq = ( r1 + r2 ) * ( r1 + r2 );
  if ( distSq == radSumSq ) return 1;
  else if ( distSq > radSumSq ) return - 1;
  else return 0;
}


int main(void) {
		f_gold(1,2,3,4,5,29);
}