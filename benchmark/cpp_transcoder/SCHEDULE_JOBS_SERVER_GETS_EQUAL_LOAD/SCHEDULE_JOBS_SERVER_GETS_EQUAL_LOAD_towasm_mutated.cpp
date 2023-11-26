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

#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a [ ], int b [ ], int n ) {
  int i;
  long int s = 0;
  for ( i = 0;
  i < n;
  i ++ ) s += ( a [ i ] + b [ i ] );
  if ( n == 1 ) return a [ 0 ] + b [ 0 ];
  if ( s % n != 0 ) return - 1;
  int x = s / n;
  for ( i = 0;
  i < n;
  i ++ ) {
    if ( a [ i ] > x ) return - 1;
    if ( i > 0 ) {
      a [ i ] += b [ i - 1 ];
      b [ i - 1 ] = 0;
    }
    if ( a [ i ] == x ) continue;
    int y = a [ i ] + b [ i ];
    if ( i + 1 < n ) y += b [ i + 1 ];
    if ( y == x ) {
      a [ i ] = y;
      b [ i ] = b [ i + 1 ] = 0;
      continue;
    }
    if ( a [ i ] + b [ i ] == x ) {
      a [ i ] += b [ i ];
      b [ i ] = 0;
      continue;
    }
    if ( i + 1 < n && a [ i ] + b [ i + 1 ] == x ) {
      a [ i ] += b [ i + 1 ];
      b [ i + 1 ] = 0;
      continue;
    }
    return - 1;
  }
  for ( i = 0;
  i < n;
  i ++ ) if ( b [ i ] != 0 ) return - 1;
  return x;
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,29);
}