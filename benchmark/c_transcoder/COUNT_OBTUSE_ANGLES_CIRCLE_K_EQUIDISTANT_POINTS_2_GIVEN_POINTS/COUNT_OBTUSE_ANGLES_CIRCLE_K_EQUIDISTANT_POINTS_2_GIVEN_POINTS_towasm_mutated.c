

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

int f_gold ( int a, int b, int k ) {
  int c1 = ( b - a ) - 1;
  int c2 = ( k - b ) + ( a - 1 );
  if ( c1 == c2 ) return 0;
  return min ( c1, c2 );
}


int main(void) {
		f_gold(1,2,29);
}