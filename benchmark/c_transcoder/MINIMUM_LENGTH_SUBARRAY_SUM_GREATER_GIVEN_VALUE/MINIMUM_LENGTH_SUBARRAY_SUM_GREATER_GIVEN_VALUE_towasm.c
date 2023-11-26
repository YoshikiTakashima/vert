

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

int f_gold ( int arr [ ], int n, int x ) {
  int curr_sum = 0, min_len = n + 1;
  int start = 0, end = 0;
  while ( end < n ) {
    while ( curr_sum <= x && end < n ) curr_sum += arr [ end ++ ];
    while ( curr_sum > x && start < n ) {
      if ( end - start < min_len ) min_len = end - start;
      curr_sum -= arr [ start ++ ];
    }
  }
  return min_len;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}