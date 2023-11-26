

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

int f_gold ( int num ) {
  int sum = 0;
  for ( int i = 2;
  i * i <= num;
  i ++ ) {
    while ( num % i == 0 ) {
      sum += i;
      num /= i;
    }
  }
  sum += num;
  return sum;
}


int main(void) {
		f_gold(29);
}