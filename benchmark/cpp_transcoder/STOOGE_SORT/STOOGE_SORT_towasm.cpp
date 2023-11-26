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
void f_gold ( int arr [ ], int l, int h ) {
  if ( l >= h ) return;
  if ( arr [ l ] > arr [ h ] ) swap ( arr [ l ], arr [ h ] );
  if ( h - l + 1 > 2 ) {
    int t = ( h - l + 1 ) / 3;
    f_gold ( arr, l, h - t );
    f_gold ( arr, l + t, h );
    f_gold ( arr, l, h - t );
  }
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);;
}