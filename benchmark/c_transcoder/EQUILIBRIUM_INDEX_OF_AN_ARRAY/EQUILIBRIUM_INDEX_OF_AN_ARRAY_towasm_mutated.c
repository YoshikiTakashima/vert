

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
  int i, j;
  int leftsum, rightsum;
  for ( i = 0;
  i < n;
  ++ i ) {
    leftsum = 0;
    for ( j = 0;
    j < i;
    j ++ ) leftsum += arr [ j ];
    rightsum = 0;
    for ( j = i + 1;
    j < n;
    j ++ ) rightsum += arr [ j ];
    if ( leftsum == rightsum ) return i;
  }
  return - 1;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}