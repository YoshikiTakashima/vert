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
int f_gold ( int arr [ ], int n, int sum ) {
  int curr_sum = arr [ 0 ], start = 0, i;
  for ( i = 1;
  i <= n;
  i ++ ) {
    while ( curr_sum > sum && start < i - 1 ) {
      curr_sum = curr_sum - arr [ start ];
      start ++;
    }
    if ( curr_sum == sum ) {
      cout << "Sum found between indexes " << start << " and " << i - 1;
      return 1;
    }
    if ( i < n ) curr_sum = curr_sum + arr [ i ];
  }
  cout << "No subarray found";
  return 0;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,29);
}