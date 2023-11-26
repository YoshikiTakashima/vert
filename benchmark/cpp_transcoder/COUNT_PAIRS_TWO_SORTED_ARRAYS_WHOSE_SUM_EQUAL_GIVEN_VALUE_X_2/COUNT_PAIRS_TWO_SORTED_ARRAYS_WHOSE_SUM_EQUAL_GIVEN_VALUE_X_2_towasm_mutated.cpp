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
int f_gold ( int arr1 [ ], int arr2 [ ], int m, int n, int x ) {
  int count = 0;
  int l = 0, r = n - 1;
  while ( l < m && r >= 0 ) {
    if ( ( arr1 [ l ] + arr2 [ r ] ) == x ) {
      l ++;
      r --;
      count ++;
    }
    else if ( ( arr1 [ l ] + arr2 [ r ] ) < x ) l ++;
    else r --;
  }
  return count;
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3,4,29);
}