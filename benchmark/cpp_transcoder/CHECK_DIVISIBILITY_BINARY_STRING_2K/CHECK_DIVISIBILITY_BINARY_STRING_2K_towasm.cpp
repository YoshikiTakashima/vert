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
int f_gold ( char str [ ], int k ) {
  int n = strlen ( str );
  int c = 0;
  for ( int i = 0;
  i < k;
  i ++ ) if ( str [ n - i - 1 ] == '0' ) c ++;
  return ( c == k );
}


int main(void) {
	char xv[] = {'a','b'};
	f_gold(xv,2);
}