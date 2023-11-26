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
int f_gold ( int arr [ ], int N ) {
  int lis [ N ];
  for ( int i = 0;
  i < N;
  i ++ ) lis [ i ] = 1;
  for ( int i = 1;
  i < N;
  i ++ ) for ( int j = 0;
  j < i;
  j ++ ) if ( arr [ i ] >= arr [ j ] && lis [ i ] < lis [ j ] + 1 ) lis [ i ] = lis [ j ] + 1;
  int max = 0;
  for ( int i = 0;
  i < N;
  i ++ ) if ( max < lis [ i ] ) max = lis [ i ];
  return ( N - max );
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}