

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
  int SubsetSum_1 = 0, SubsetSum_2 = 0;
  for ( int i = 0;
  i <= n - 1;
  i ++ ) {
    bool isSingleOccurance = 1;
    for ( int j = i + 1;
    j <= n - 1;
    j ++ ) {
      if ( arr [ i ] == arr [ j ] ) {
        isSingleOccurance = 0;
        arr [ i ] = arr [ j ] = 0;
        break;
      }
    }
    if ( isSingleOccurance ) {
      if ( arr [ i ] > 0 ) SubsetSum_1 += arr [ i ];
      else SubsetSum_2 += arr [ i ];
    }
  }
  return abs ( SubsetSum_1 - SubsetSum_2 );
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}