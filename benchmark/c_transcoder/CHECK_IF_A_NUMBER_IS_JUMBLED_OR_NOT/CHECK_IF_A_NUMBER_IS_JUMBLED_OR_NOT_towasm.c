

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

int f_gold ( int num ) {
  if ( num / 10 == 0 ) return 1;
  while ( num != 0 ) {
    if ( num / 10 == 0 ) return 1;
    int digit1 = num % 10;
    int digit2 = ( num / 10 ) % 10;
    if ( abs ( digit2 - digit1 ) > 1 ) return 0;
    num = num / 10;
  }
  return 1;
}


int main(void) {
		f_gold(1);
}