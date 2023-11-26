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
int f_gold ( int m, int n ) {
  int count [ m ] [ n ];
  for ( int i = 0;
  i < m;
  i ++ ) count [ i ] [ 0 ] = 1;
  for ( int j = 0;
  j < n;
  j ++ ) count [ 0 ] [ j ] = 1;
  for ( int i = 1;
  i < m;
  i ++ ) {
    for ( int j = 1;
    j < n;
    j ++ ) count [ i ] [ j ] = count [ i - 1 ] [ j ] + count [ i ] [ j - 1 ];
  }
  return count [ m - 1 ] [ n - 1 ];
}


int main(void) {
		f_gold(1,29);
}