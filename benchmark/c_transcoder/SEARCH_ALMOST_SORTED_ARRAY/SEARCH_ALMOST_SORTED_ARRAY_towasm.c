

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

int f_gold ( int arr [ ], int l, int r, int x ) {
  if ( r >= l ) {
    int mid = l + ( r - l ) / 2;
    if ( arr [ mid ] == x ) return mid;
    if ( mid > l && arr [ mid - 1 ] == x ) return ( mid - 1 );
    if ( mid < r && arr [ mid + 1 ] == x ) return ( mid + 1 );
    if ( arr [ mid ] > x ) return f_gold ( arr, l, mid - 2, x );
    return f_gold ( arr, mid + 2, r, x );
  }
  return - 1;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3,4);
}