

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

int f_gold ( int a, int b, int c ) {
  if ( a > b ) {
    if ( b > c ) return b;
    else if ( a > c ) return c;
    else return a;
  }
  else {
    if ( a > c ) return a;
    else if ( b > c ) return c;
    else return b;
  }
}


int main(void) {
		f_gold(1,2,3);
}