

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
  int first = 1, second = 1, number = 2, next = 1;
  while ( next ) {
    next = ( first + second ) % p;
    first = second;
    second = next;
    number ++;
  }
  return number;
}


int main(void) {
		f_gold(1);
}