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
int f_gold ( unsigned int n, unsigned int k ) {
  bool oneSeen = 0;
  while ( n > 0 ) {
    int digit = n % k;
    if ( digit > 1 ) return 0;
    if ( digit == 1 ) {
      if ( oneSeen ) return 0;
      oneSeen = 1;
    }
    n /= k;
  }
  return 1;
}


int main(void) {
		f_gold(1,2);
}