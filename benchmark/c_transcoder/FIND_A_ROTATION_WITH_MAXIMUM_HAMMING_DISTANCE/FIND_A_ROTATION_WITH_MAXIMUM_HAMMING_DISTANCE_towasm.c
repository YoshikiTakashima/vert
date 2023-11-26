

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

int f_gold ( int arr [ ], int n ) {
  int brr [ 2 * n + 1 ];
  for ( int i = 0;
  i < n;
  i ++ ) brr [ i ] = arr [ i ];
  for ( int i = 0;
  i < n;
  i ++ ) brr [ n + i ] = arr [ i ];
  int maxHam = 0;
  for ( int i = 1;
  i < n;
  i ++ ) {
    int currHam = 0;
    for ( int j = i, k = 0;
    j < ( i + n );
    j ++, k ++ ) if ( brr [ j ] != arr [ k ] ) currHam ++;
    if ( currHam == n ) return n;
    maxHam = max ( maxHam, currHam );
  }
  return maxHam;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}