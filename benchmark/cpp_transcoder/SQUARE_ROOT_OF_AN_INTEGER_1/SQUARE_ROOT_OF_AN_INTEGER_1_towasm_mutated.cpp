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
int f_gold ( int x ) {
  if ( x == 0 || x == 1 ) return x;
  int start = 1, end = x, ans;
  while ( start <= end ) {
    int mid = ( start + end ) / 2;
    if ( mid * mid == x ) return mid;
    if ( mid * mid < x ) {
      start = mid + 1;
      ans = mid;
    }
    else end = mid - 1;
  }
  return ans;
}


int main(void) {
		f_gold(29);
}