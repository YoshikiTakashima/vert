

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

int f_gold ( int a, int b ) {
  int count = 0;
  int p = abs ( a * b );
  if ( p == 0 ) return 1;
  while ( p > 0 ) {
    count ++;
    p = p / 10;
  }
  return count;
}


int main(void) {
		f_gold(1,29);
}