

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

int f_gold ( int p ) {
  long long checkNumber = pow ( 2, p ) - 1;
  long long nextval = 4 % checkNumber;
  for ( int i = 1;
  i < p - 1;
  i ++ ) nextval = ( nextval * nextval - 2 ) % checkNumber;
  if(nextval == 0) return 1;
  else return 0;
}


int main(void) {
		f_gold(1);
}