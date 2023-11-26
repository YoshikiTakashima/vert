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
  int lioes [ n ];
  int maxLen = 0;
  for ( int i = 0;
  i < n;
  i ++ ) lioes [ i ] = 1;
  for ( int i = 1;
  i < n;
  i ++ ) for ( int j = 0;
  j < i;
  j ++ ) if ( arr [ i ] > arr [ j ] && ( arr [ i ] + arr [ j ] ) % 2 != 0 && lioes [ i ] < lioes [ j ] + 1 ) lioes [ i ] = lioes [ j ] + 1;
  for ( int i = 0;
  i < n;
  i ++ ) if ( maxLen < lioes [ i ] ) maxLen = lioes [ i ];
  return maxLen;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}