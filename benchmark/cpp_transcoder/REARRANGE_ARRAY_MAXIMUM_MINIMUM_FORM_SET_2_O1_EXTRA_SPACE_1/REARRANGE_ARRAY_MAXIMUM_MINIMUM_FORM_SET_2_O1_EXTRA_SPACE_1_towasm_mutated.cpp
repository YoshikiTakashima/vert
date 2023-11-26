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
  int max_ele = arr [ n - 1 ];
  int min_ele = arr [ 0 ];
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( i % 2 == 0 ) {
      arr [ i ] = max_ele;
      max_ele -= 1;
    }
    else {
      arr [ i ] = min_ele;
      min_ele += 1;
    }
  }
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);;
}