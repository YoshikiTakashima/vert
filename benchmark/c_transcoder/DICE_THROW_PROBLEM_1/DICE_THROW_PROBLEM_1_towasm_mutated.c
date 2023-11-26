

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

long f_gold ( int f, int d, int s ) {
  long mem [ d + 1 ] [ s + 1 ];
  memset ( mem, 0, sizeof mem );
  mem [ 0 ] [ 0 ] = 1;
  for ( int i = 1;
  i <= d;
  i ++ ) {
    for ( int j = i;
    j <= s;
    j ++ ) {
      mem [ i ] [ j ] = mem [ i ] [ j - 1 ] + mem [ i - 1 ] [ j - 1 ];
      if ( j - f - 1 >= 0 ) mem [ i ] [ j ] -= mem [ i - 1 ] [ j - f - 1 ];
    }
  }
  return mem [ d ] [ s ];
}


int main(void) {
		f_gold(1,2,29);
}