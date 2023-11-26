

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

int f_gold ( int a, int b ) {
  if ( b == 0 ) return 1;
  int answer = a;
  int increment = a;
  int i, j;
  for ( i = 1;
  i < b;
  i ++ ) {
    for ( j = 1;
    j < a;
    j ++ ) {
      answer += increment;
    }
    increment = answer;
  }
  return answer;
}


int main(void) {
		f_gold(1,2);
}