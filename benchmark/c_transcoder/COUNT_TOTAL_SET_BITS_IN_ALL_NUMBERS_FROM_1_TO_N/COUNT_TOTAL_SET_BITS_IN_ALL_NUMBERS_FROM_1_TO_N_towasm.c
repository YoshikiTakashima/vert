

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
  int i = 0;
  int ans = 0;
  while ( ( 1 << i ) <= n ) {
    bool k = 0;
    int change = 1 << i;
    for ( int j = 0;
    j <= n;
    j ++ ) {
      ans += k;
      if ( change == 1 ) {
        k = ! k;
        change = 1 << i;
      }
      else {
        change --;
      }
    }
    i ++;
  }
  return ans;
}


int main(void) {
		f_gold(1);
}