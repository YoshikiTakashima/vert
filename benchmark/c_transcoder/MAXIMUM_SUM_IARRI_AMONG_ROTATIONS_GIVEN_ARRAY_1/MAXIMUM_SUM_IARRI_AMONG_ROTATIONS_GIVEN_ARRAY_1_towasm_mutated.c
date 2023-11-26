

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
  int cum_sum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) cum_sum += arr [ i ];
  int curr_val = 0;
  for ( int i = 0;
  i < n;
  i ++ ) curr_val += i * arr [ i ];
  int res = curr_val;
  for ( int i = 1;
  i < n;
  i ++ ) {
    int next_val = curr_val - ( cum_sum - arr [ i - 1 ] ) + arr [ i - 1 ] * ( n - 1 );
    curr_val = next_val;
    res = max ( res, next_val );
  }
  return res;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}