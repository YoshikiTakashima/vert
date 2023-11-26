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
int f_gold ( double h, double m ) {
  if ( h < 0 || m < 0 || h > 12 || m > 60 ) printf ( "Wrong input" );
  if ( h == 12 ) h = 0;
  if ( m == 60 ) m = 0;
  int hour_angle = 0.5 * ( h * 60 + m );
  int minute_angle = 6 * m;
  int angle = abs ( hour_angle - minute_angle );
  angle = min ( 360 - angle, angle );
  return angle;
}


int main(void) {
		f_gold(1,2);
}