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
int f_gold ( int a [ ], int n ) {
  sort ( a, a + n );
  int count = 1;
  int answer = 0;
  for ( int i = 1;
  i < n;
  i ++ ) {
    if ( a [ i ] == a [ i - 1 ] ) {
      count += 1;
    }
    else {
      answer = answer + ( count * ( count - 1 ) ) / 2;
      count = 1;
    }
  }
  answer = answer + ( count * ( count - 1 ) ) / 2;
  return answer;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}