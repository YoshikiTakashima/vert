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
int f_gold ( int n, int templeHeight [ ] ) {
  int sum = 0;
  for ( int i = 0;
  i < n;
  ++ i ) {
    int left = 0, right = 0;
    for ( int j = i - 1;
    j >= 0;
    -- j ) {
      if ( templeHeight [ j ] < templeHeight [ j + 1 ] ) ++ left;
      else break;
    }
    for ( int j = i + 1;
    j < n;
    ++ j ) {
      if ( templeHeight [ j ] < templeHeight [ j - 1 ] ) ++ right;
      else break;
    }
    sum += max ( right, left ) + 1;
  }
  return sum;
}


int main(void) {
		int yq[] = {2929,292};
	f_gold(29,yq);
}