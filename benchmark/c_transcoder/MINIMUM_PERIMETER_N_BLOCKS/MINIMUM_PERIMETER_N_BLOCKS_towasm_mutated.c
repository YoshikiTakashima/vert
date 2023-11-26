

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

int f_gold ( int n ) {
  int l = sqrt ( n );
  int sq = l * l;
  if ( sq == n ) return l * 4;
  else {
    long int row = n / l;
    long int perimeter = 2 * ( l + row );
    if ( n % l != 0 ) perimeter += 2;
    return perimeter;
  }
}


int main(void) {
		f_gold(29);
}