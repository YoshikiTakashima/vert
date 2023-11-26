

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
  int invcount = 0;
  for ( int i = 1;
  i < n - 1;
  i ++ ) {
    int small = 0;
    for ( int j = i + 1;
    j < n;
    j ++ ) if ( arr [ i ] > arr [ j ] ) small ++;
    int great = 0;
    for ( int j = i - 1;
    j >= 0;
    j -- ) if ( arr [ i ] < arr [ j ] ) great ++;
    invcount += great * small;
  }
  return invcount;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}