

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
  int prime [ n + 1 ], sum = 0;
  memset ( prime, 0, sizeof ( prime ) );
  int max = n / 2;
  for ( int p = 2;
  p <= max;
  p ++ ) {
    if ( prime [ p ] == 0 ) {
      for ( int i = p * 2;
      i <= n;
      i += p ) prime [ i ] = p;
    }
  }
  for ( int p = 2;
  p <= n;
  p ++ ) {
    if ( prime [ p ] ) sum += prime [ p ];
    else sum += p;
  }
  return sum;
}


int main(void) {
		f_gold(1);
}