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
  if ( n < 3 ) return n;
  if ( n >= 3 && n < 10 ) return n - 1;
  int po = 1;
  while ( n / po > 9 ) po = po * 10;
  int msd = n / po;
  if ( msd != 3 ) return f_gold ( msd ) * f_gold ( po - 1 ) + f_gold ( msd ) + f_gold ( n % po );
  else return f_gold ( msd * po - 1 );
}


int main(void) {
		f_gold(1);
}