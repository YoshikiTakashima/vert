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
void f_gold ( int arr [ ], int n, int k ) {
  for ( int i = 0;
  i < k;
  i ++ ) {
    int x = arr [ 0 ];
    for ( int j = 0;
    j < n - 1;
    ++ j ) arr [ j ] = arr [ j + 1 ];
    arr [ n - 1 ] = x;
  }
}



int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);;
}