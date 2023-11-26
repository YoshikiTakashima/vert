

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

int f_gold ( int x ) {
  int temp = x;
  int n = 0;
  while ( x != 0 ) {
    x /= 10;
    n ++;
  }
  x = temp;
  int sum = 0;
  while ( x != 0 ) {
    sum += pow ( x % 10, n );
    x /= 10;
  }
  return ( sum == temp );
}


int main(void) {
		f_gold(1);
}