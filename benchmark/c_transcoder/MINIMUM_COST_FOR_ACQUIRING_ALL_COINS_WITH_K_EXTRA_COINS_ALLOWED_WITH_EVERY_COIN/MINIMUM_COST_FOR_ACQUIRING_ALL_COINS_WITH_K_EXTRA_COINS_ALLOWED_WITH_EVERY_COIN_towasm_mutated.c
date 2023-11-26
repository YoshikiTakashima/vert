

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

int f_gold ( int coin [ ], int n, int k ) {
  sort ( coin, coin + n );
  int coins_needed = ceil ( 1.0 * n / ( k + 1 ) );
  int ans = 0;
  for ( int i = 0;
  i <= coins_needed - 1;
  i ++ ) ans += coin [ i ];
  return ans;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,29);
}