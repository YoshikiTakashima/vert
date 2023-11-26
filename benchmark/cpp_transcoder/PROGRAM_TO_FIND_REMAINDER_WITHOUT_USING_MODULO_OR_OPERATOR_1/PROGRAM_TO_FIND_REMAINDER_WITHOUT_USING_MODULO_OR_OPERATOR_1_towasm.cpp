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
int f_gold ( int num, int divisor ) {
  if ( divisor == 0 ) {
    cout << "Error: divisor can't be zero \n";
    return - 1;
  }
  if ( divisor < 0 ) divisor = - divisor;
  if ( num < 0 ) num = - num;
  int i = 1;
  int product = 0;
  while ( product <= num ) {
    product = divisor * i;
    i ++;
  }
  return num - ( product - divisor );
}


int main(void) {
		f_gold(1,2);
}