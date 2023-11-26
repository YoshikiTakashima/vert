

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
  sort ( arr, arr + n );
  int max_count = 1, res = arr [ 0 ], curr_count = 1;
  for ( int i = 1;
  i < n;
  i ++ ) {
    if ( arr [ i ] == arr [ i - 1 ] ) curr_count ++;
    else {
      if ( curr_count > max_count ) {
        max_count = curr_count;
        res = arr [ i - 1 ];
      }
      curr_count = 1;
    }
  }
  if ( curr_count > max_count ) {
    max_count = curr_count;
    res = arr [ n - 1 ];
  }
  return res;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}