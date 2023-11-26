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
void f_gold ( int arr [ ], int n ) {
  int max_idx = n - 1, min_idx = 0;
  int max_elem = arr [ n - 1 ] + 1;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( i % 2 == 0 ) {
      arr [ i ] += ( arr [ max_idx ] % max_elem ) * max_elem;
      max_idx --;
    }
    else {
      arr [ i ] += ( arr [ min_idx ] % max_elem ) * max_elem;
      min_idx ++;
    }
  }
  for ( int i = 0;
  i < n;
  i ++ ) arr [ i ] = arr [ i ] / max_elem;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);;
}