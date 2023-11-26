

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
  int count = 0, ans = 1;
  while ( n % 2 == 0 ) {
    count ++;
    n /= 2;
  }
  if ( count % 2 ) ans *= 2;
  for ( int i = 3;
  i <= sqrt ( n );
  i += 2 ) {
    count = 0;
    while ( n % i == 0 ) {
      count ++;
      n /= i;
    }
    if ( count % 2 ) ans *= i;
  }
  if ( n > 2 ) ans *= n;
  return ans;
}


int main(void) {
		f_gold(29);
}