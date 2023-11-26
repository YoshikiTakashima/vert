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
int f_gold ( int array [ ], int start, int end ) {
  if ( start > end ) return end + 1;
  if ( start != array [ start ] ) return start;
  int mid = ( start + end ) / 2;
  if ( array [ mid ] == mid ) return f_gold ( array, mid + 1, end );
  return f_gold ( array, start, mid );
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,29);
}