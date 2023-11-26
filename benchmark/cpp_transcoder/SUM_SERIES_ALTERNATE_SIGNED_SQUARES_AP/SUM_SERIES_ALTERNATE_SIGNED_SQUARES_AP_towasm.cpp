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
int f_gold ( int n, int a [ ] ) {
  int res = 0;
  for ( int i = 0;
  i < 2 * n;
  i ++ ) {
    if ( i % 2 == 0 ) res += a [ i ] * a [ i ];
    else res -= a [ i ] * a [ i ];
  }
  return res;
}


int main(void) {
		int yq[] = {11,12};
	f_gold(1,yq);
}