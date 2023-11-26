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
  int max = * max_element ( arr, arr + n );
  int min = * min_element ( arr, arr + n );
  int m = max - min + 1;
  if ( m > n ) return 0;
  bool visited [ m ];
  memset ( visited, 0, sizeof ( visited ) );
  for ( int i = 0;
  i < n;
  i ++ ) visited [ arr [ i ] - min ] = 1;
  for ( int i = 0;
  i < m;
  i ++ ) if ( visited [ i ] == 0 ) return 0;
  return 1;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}