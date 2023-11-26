

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
  int a = 1, b = 2, c = 0;
  if ( n <= 2 ) {
    return n;
  }
  for ( int i = 3;
  i <= n;
  i ++ ) {
    c = b + ( i - 1 ) * a;
    a = b;
    b = c;
  }
  return c;
}


int main(void) {
		f_gold(1);
}