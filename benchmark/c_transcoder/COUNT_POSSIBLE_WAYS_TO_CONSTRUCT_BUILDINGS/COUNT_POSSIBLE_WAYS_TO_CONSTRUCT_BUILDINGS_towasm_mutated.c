

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

int f_gold ( int N ) {
  if ( N == 1 ) return 4;
  int countB = 1, countS = 1, prev_countB, prev_countS;
  for ( int i = 2;
  i <= N;
  i ++ ) {
    prev_countB = countB;
    prev_countS = countS;
    countS = prev_countB + prev_countS;
    countB = prev_countS;
  }
  int result = countS + countB;
  return ( result * result );
}


int main(void) {
		f_gold(29);
}