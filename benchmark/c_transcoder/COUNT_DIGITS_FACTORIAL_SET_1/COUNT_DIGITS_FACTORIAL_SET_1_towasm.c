

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

int f_gold ( int n ) {
  if ( n < 0 ) return 0;
  if ( n <= 1 ) return 1;
  double digits = 0;
  for ( int i = 2;
  i <= n;
  i ++ ) digits += log10 ( i );
  return floor ( digits ) + 1;
}


int main(void) {
		f_gold(1);
}