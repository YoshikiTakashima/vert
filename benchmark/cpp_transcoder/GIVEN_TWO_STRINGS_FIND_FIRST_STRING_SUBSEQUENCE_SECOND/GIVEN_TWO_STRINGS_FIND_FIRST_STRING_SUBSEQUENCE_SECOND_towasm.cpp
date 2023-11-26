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
int f_gold ( char str1 [ ], char str2 [ ], int m, int n ) {
  if ( m == 0 ) return 1;
  if ( n == 0 ) return 0;
  if ( str1 [ m - 1 ] == str2 [ n - 1 ] ) return f_gold ( str1, str2, m - 1, n - 1 );
  return f_gold ( str1, str2, m, n - 1 );
}


int main(void) {
	char xv[] = {'a','b'};
char yq[] = {'a','b'};
	f_gold(xv,yq,3,4);
}