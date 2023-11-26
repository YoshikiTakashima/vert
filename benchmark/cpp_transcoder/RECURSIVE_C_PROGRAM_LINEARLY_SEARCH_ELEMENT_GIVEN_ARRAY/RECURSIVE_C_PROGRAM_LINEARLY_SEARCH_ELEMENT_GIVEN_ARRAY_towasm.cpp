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
int f_gold ( int arr [ ], int l, int r, int x ) {
  if ( r < l ) return - 1;
  if ( arr [ l ] == x ) return l;
  if ( arr [ r ] == x ) return r;
  return f_gold ( arr, l + 1, r - 1, x );
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3,4);
}