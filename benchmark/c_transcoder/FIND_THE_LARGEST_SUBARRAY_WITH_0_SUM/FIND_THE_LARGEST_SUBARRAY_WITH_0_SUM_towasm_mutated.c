

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
  int max_len = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    int curr_sum = 0;
    for ( int j = i;
    j < n;
    j ++ ) {
      curr_sum += arr [ j ];
      if ( curr_sum == 0 ) max_len = max ( max_len, j - i + 1 );
    }
  }
  return max_len;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}