

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
  int result = 0;
  for ( int i = 2;
  i <= sqrt ( n );
  i ++ ) {
    if ( n % i == 0 ) {
      if ( i == ( n / i ) ) result += i;
      else result += ( i + n / i );
    }
  }
  return ( result + n + 1 );
}


int main(void) {
		f_gold(29);
}