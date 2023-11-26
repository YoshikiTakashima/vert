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
int f_gold ( int n ) {
  int ans = 0, temp = 0, num;
  for ( int i = 1;
  i <= n && temp < n;
  i ++ ) {
    temp = i - 1;
    num = 1;
    while ( temp < n ) {
      if ( temp + i <= n ) ans += ( i * num );
      else ans += ( ( n - temp ) * num );
      temp += i;
      num ++;
    }
  }
  return ans;
}


int main(void) {
		f_gold(29);
}