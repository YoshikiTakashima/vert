

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

int f_gold ( int dist ) {
  int count [ dist + 1 ];
  count [ 0 ] = 1, count [ 1 ] = 1, count [ 2 ] = 2;
  for ( int i = 3;
  i <= dist;
  i ++ ) count [ i ] = count [ i - 1 ] + count [ i - 2 ] + count [ i - 3 ];
  return count [ dist ];
}


int main(void) {
		f_gold(1);
}