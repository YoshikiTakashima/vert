

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

int f_gold ( int n, int k ) {
  while ( n % 2 == 0 ) {
    k --;
    n = n / 2;
    if ( k == 0 ) return 2;
  }
  for ( int i = 3;
  i <= sqrt ( n );
  i = i + 2 ) {
    while ( n % i == 0 ) {
      if ( k == 1 ) return i;
      k --;
      n = n / i;
    }
  }
  if ( n > 2 && k == 1 ) return n;
  return - 1;
}


int main(void) {
		f_gold(1,29);
}