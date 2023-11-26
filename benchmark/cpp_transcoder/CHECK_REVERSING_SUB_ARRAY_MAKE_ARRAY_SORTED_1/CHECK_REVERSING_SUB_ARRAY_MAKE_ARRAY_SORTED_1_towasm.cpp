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
int f_gold ( int arr [ ], int n ) {
  if ( n == 1 ) return 1;
  int i;
  for ( i = 1;
  i < n && arr [ i - 1 ] < arr [ i ];
  i ++ );
  if ( i == n ) return 1;
  int j = i;
  while ( arr [ j ] < arr [ j - 1 ] ) {
    if ( i > 1 && arr [ j ] < arr [ i - 2 ] ) return 0;
    j ++;
  }
  if ( j == n ) return 1;
  int k = j;
  if ( arr [ k ] < arr [ i - 1 ] ) return 0;
  while ( k > 1 && k < n ) {
    if ( arr [ k ] < arr [ k - 1 ] ) return 0;
    k ++;
  }
  return 1;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}