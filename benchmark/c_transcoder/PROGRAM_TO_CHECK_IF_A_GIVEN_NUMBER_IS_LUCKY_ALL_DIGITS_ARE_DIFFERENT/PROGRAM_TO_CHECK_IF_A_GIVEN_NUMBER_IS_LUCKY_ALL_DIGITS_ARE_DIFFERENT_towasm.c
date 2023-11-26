

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
  bool arr [ 10 ];
  for ( int i = 0;
  i < 10;
  i ++ ) arr [ i ] = 0;
  while ( n > 0 ) {
    int digit = n % 10;
    if ( arr [ digit ] ) return 0;
    arr [ digit ] = 1;
    n = n / 10;
  }
  return 1;
}


int main(void) {
		f_gold(1);
}