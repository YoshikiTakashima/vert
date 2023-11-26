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
int f_gold ( int k, int x [ ], int y [ ], int n ) {
  int dis [ n ];
  for ( int i = 0;
  i < n;
  i ++ ) dis [ i ] = x [ i ] * x [ i ] + y [ i ] * y [ i ];
  sort ( dis, dis + n );
  return dis [ k - 1 ];
}


int main(void) {
		int yq[] = {11,12};
	int qe[] = {11,12};
	f_gold(1,yq,qe,29);
}