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
int f_gold ( int A [ ], int arr_size, int sum ) {
  int l, r;
  sort ( A, A + arr_size );
  for ( int i = 0;
  i < arr_size - 2;
  i ++ ) {
    l = i + 1;
    r = arr_size - 1;
    while ( l < r ) {
      if ( A [ i ] + A [ l ] + A [ r ] == sum ) {
        printf ( "Triplet is %d, %d, %d", A [ i ], A [ l ], A [ r ] );
        return 1;
      }
      else if ( A [ i ] + A [ l ] + A [ r ] < sum ) l ++;
      else r --;
    }
  }
  return 0;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}