

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

int f_gold ( int arr1 [ ], int arr2 [ ], int n ) {
  int index = n;
  int left = 0, right = n - 1;
  while ( left <= right ) {
    int mid = ( left + right ) / 2;
    if ( arr2 [ mid ] == arr1 [ mid ] ) left = mid + 1;
    else {
      index = mid;
      right = mid - 1;
    }
  }
  return index;
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,29);
}