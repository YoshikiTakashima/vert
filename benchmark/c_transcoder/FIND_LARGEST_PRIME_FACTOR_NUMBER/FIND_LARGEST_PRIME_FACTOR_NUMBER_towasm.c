

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

long long f_gold ( long long n ) {
  long long maxPrime = - 1;
  while ( n % 2 == 0 ) {
    maxPrime = 2;
    n >>= 1;
  }
  for ( int i = 3;
  i <= sqrt ( n );
  i += 2 ) {
    while ( n % i == 0 ) {
      maxPrime = i;
      n = n / i;
    }
  }
  if ( n > 2 ) maxPrime = n;
  return maxPrime;
}


int main(void) {
		f_gold(1);
}