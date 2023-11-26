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
int f_gold ( int arr [ ], int low, int high ) {
  if ( low > high ) return - 1;
  int mid = ( low + high ) / 2;
  if ( arr [ mid ] != mid + 1 ) {
    if ( mid > 0 && arr [ mid ] == arr [ mid - 1 ] ) return mid;
    return f_gold ( arr, low, mid - 1 );
  }
  return f_gold ( arr, mid + 1, high );
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}