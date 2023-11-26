

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

int f_gold ( int r, int R, int r1, int x1, int y1 ) {
  int dis = sqrt ( x1 * x1 + y1 * y1 );
  return ( dis - r1 >= R && dis + r1 <= r );
}


int main(void) {
		f_gold(1,2,3,4,5);
}