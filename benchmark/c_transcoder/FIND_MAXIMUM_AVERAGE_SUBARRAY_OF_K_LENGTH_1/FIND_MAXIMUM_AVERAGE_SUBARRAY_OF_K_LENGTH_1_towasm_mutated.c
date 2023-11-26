

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

int f_gold ( int arr [ ], int n, int k ) {
  if ( k > n ) return - 1;
  int sum = arr [ 0 ];
  for ( int i = 1;
  i < k;
  i ++ ) sum += arr [ i ];
  int max_sum = sum, max_end = k - 1;
  for ( int i = k;
  i < n;
  i ++ ) {
    int sum = sum + arr [ i ] - arr [ i - k ];
    if ( sum > max_sum ) {
      max_sum = sum;
      max_end = i;
    }
  }
  return max_end - k + 1;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,29);
}