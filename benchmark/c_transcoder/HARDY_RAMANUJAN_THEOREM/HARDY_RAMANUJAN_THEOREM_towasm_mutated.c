

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
  int count = 0;
  if ( n % 2 == 0 ) {
    count ++;
    while ( n % 2 == 0 ) n = n / 2;
  }
  for ( int i = 3;
  i <= sqrt ( n );
  i = i + 2 ) {
    if ( n % i == 0 ) {
      count ++;
      while ( n % i == 0 ) n = n / i;
    }
  }
  if ( n > 2 ) count ++;
  return count;
}


int main(void) {
		f_gold(29);
}