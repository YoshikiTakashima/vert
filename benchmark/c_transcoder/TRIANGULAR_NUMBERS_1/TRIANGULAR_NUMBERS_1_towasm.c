

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

int f_gold ( int num ) {
  if ( num < 0 ) return 0;
  int c = ( - 2 * num );
  int b = 1, a = 1;
  int d = ( b * b ) - ( 4 * a * c );
  if ( d < 0 ) return 0;
  float root1 = ( - b + sqrt ( d ) ) / ( 2 * a );
  float root2 = ( - b - sqrt ( d ) ) / ( 2 * a );
  if ( root1 > 0 && floor ( root1 ) == root1 ) return 1;
  if ( root2 > 0 && floor ( root2 ) == root2 ) return 1;
  return 0;
}


int main(void) {
		f_gold(1);
}