

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

char f_gold ( char strA [], char strB [] ) {
  int res = 0, i;
  for ( i = 0;
  i < strlen(strA);
  i ++ ) {
    res ^= strA [ i ];
  }
  for ( i = 0;
  i < strlen(strB);
  i ++ ) {
    res ^= strB [ i ];
  }
  return ( ( char ) ( res ) );
}


int main(void) {
	char xv[] = {'ac','cd'};
char yq[] = {'ac','cd'};
	f_gold(xv,yq);
}