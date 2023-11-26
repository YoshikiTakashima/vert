

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
  if ( n % 2 != 0 ) return 0;
  int res = 1;
  for ( int i = 2;
  i <= sqrt ( n );
  i ++ ) {
    int count = 0, curr_sum = 1, curr_term = 1;
    while ( n % i == 0 ) {
      count ++;
      n = n / i;
      if ( i == 2 && count == 1 ) curr_sum = 0;
      curr_term *= i;
      curr_sum += curr_term;
    }
    res *= curr_sum;
  }
  if ( n >= 2 ) res *= ( 1 + n );
  return res;
}


int main(void) {
		f_gold(29);
}