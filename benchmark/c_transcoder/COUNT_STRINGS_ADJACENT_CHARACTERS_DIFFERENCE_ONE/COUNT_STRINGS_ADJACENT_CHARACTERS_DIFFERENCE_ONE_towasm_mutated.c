

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
  int dp [ n + 1 ] [ 27 ];
  memset ( dp, 0, sizeof ( dp ) );
  for ( int i = 0;
  i <= 25;
  i ++ ) dp [ 1 ] [ i ] = 1;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    for ( int j = 0;
    j <= 25;
    j ++ ) if ( j == 0 ) dp [ i ] [ j ] = dp [ i - 1 ] [ j + 1 ];
    else dp [ i ] [ j ] = ( dp [ i - 1 ] [ j - 1 ] + dp [ i - 1 ] [ j + 1 ] );
  }
  int sum = 0;
  for ( int i = 0;
  i <= 25;
  i ++ ) sum = ( sum + dp [ n ] [ i ] );
  return sum;
}


int main(void) {
		f_gold(29);
}