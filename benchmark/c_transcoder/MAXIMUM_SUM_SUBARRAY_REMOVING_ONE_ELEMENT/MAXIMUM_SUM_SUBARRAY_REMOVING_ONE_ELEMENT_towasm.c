

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
  int fw [ n ], bw [ n ];
  int cur_max = arr [ 0 ], max_so_far = arr [ 0 ];
  fw [ 0 ] = arr [ 0 ];
  for ( int i = 1;
  i < n;
  i ++ ) {
    cur_max = max ( arr [ i ], cur_max + arr [ i ] );
    max_so_far = max ( max_so_far, cur_max );
    fw [ i ] = cur_max;
  }
  cur_max = max_so_far = bw [ n - 1 ] = arr [ n - 1 ];
  for ( int i = n - 2;
  i >= 0;
  i -- ) {
    cur_max = max ( arr [ i ], cur_max + arr [ i ] );
    max_so_far = max ( max_so_far, cur_max );
    bw [ i ] = cur_max;
  }
  int fans = max_so_far;
  for ( int i = 1;
  i < n - 1;
  i ++ ) fans = max ( fans, fw [ i - 1 ] + bw [ i + 1 ] );
  return fans;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}