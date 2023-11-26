

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

int f_gold ( char s1 [], char s2 [] ) {
  int n = strlen(s1);
  int m = strlen(s2);
  bool dp [ n + 1 ] [ m + 1 ];
  for ( int i = 0;
  i <= n;
  i ++ ) {
    for ( int j = 0;
    j <= m;
    j ++ ) {
      dp [ i ] [ j ] = 0;
    }
  }
  dp [ 0 ] [ 0 ] = 1;
  for ( int i = 0;
  i < strlen(s1);
  i ++ ) {
    for ( int j = 0;
    j <= strlen(s2);
    j ++ ) {
      if ( dp [ i ] [ j ] ) {
        if ( j < strlen(s2) && ( toupper ( s1 [ i ] ) == s2 [ j ] ) ) dp [ i + 1 ] [ j + 1 ] = 1;
        if ( ! isupper ( s1 [ i ] ) ) dp [ i + 1 ] [ j ] = 1;
      }
    }
  }
  return ( dp [ n ] [ m ] );
}


int main(void) {
	char xv[] = {'a','d'};
char yq[] = {'a','d'};
	f_gold(xv,yq);
}