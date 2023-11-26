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
int f_gold ( int arr [ ], int n, int num, int maxLimit ) {
  int ind;
  int val;
  int dp [ n ] [ maxLimit + 1 ];
  for ( ind = 0;
  ind < n;
  ind ++ ) {
    for ( val = 0;
    val <= maxLimit;
    val ++ ) {
      if ( ind == 0 ) {
        if ( num - arr [ ind ] == val || num + arr [ ind ] == val ) {
          dp [ ind ] [ val ] = 1;
        }
        else {
          dp [ ind ] [ val ] = 0;
        }
      }
      else {
        if ( val - arr [ ind ] >= 0 && val + arr [ ind ] <= maxLimit ) {
          dp [ ind ] [ val ] = dp [ ind - 1 ] [ val - arr [ ind ] ] || dp [ ind - 1 ] [ val + arr [ ind ] ];
        }
        else if ( val - arr [ ind ] >= 0 ) {
          dp [ ind ] [ val ] = dp [ ind - 1 ] [ val - arr [ ind ] ];
        }
        else if ( val + arr [ ind ] <= maxLimit ) {
          dp [ ind ] [ val ] = dp [ ind - 1 ] [ val + arr [ ind ] ];
        }
        else {
          dp [ ind ] [ val ] = 0;
        }
      }
    }
  }
  for ( val = maxLimit;
  val >= 0;
  val -- ) {
    if ( dp [ n - 1 ] [ val ] ) {
      return val;
    }
  }
  return - 1;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3,4);
}