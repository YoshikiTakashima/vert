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
  int j = 0;
  for ( int i = 0;
  i < n && j < m;
  i ++ ) if ( str1 [ j ] == str2 [ i ] ) j ++;
  return ( j == m );
}


int main(void) {
	char xv[] = {'c','d'};
char yq[] = {'c','d'};
	f_gold(xv,yq,3,29);
}