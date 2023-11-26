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
  for ( int i = 0;
  i < n;
  i ++ ) if ( arr [ i ] & 1 ) arr [ i ] *= - 1;
  sort ( arr, arr + n );
  for ( int i = 0;
  i < n;
  i ++ ) if ( arr [ i ] & 1 ) arr [ i ] *= - 1;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);;
}