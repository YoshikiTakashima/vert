

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
  int longest_start = - 1, longest_end = 0;
  for ( int start = 0;
  start < n;
  start ++ ) {
    int min = INT_MAX, max = INT_MIN;
    for ( int end = start;
    end < n;
    end ++ ) {
      int val = arr [ end ];
      if ( val < min ) min = val;
      if ( val > max ) max = val;
      if ( 2 * min <= max ) break;
      if ( end - start > longest_end - longest_start || longest_start == - 1 ) {
        longest_start = start;
        longest_end = end;
      }
    }
  }
  if ( longest_start == - 1 ) return n;
  return ( n - ( longest_end - longest_start + 1 ) );
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}