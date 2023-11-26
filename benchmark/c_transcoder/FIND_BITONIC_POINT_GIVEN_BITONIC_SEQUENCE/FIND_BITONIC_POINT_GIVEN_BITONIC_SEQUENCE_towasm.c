

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

int f_gold ( int arr [ ], int left, int right ) {
  if ( left <= right ) {
    int mid = ( left + right ) / 2;
    if ( arr [ mid - 1 ] < arr [ mid ] && arr [ mid ] > arr [ mid + 1 ] ) return mid;
    if ( arr [ mid ] < arr [ mid + 1 ] ) return f_gold ( arr, mid + 1, right );
    else return f_gold ( arr, left, mid - 1 );
  }
  return - 1;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}