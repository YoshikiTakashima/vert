

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
  int temp [ n ];
  for ( int i = 0;
  i < n;
  i ++ ) temp [ i ] = arr [ i ];
  sort ( temp, temp + n );
  int front;
  for ( front = 0;
  front < n;
  front ++ ) if ( temp [ front ] != arr [ front ] ) break;
  int back;
  for ( back = n - 1;
  back >= 0;
  back -- ) if ( temp [ back ] != arr [ back ] ) break;
  if ( front >= back ) return 1;
  do {
    front ++;
    if ( arr [ front - 1 ] < arr [ front ] ) return 0;
  }
  while ( front != back );
  return 1;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}