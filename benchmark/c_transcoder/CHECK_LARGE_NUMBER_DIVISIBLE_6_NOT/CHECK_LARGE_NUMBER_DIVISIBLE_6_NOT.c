// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//


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

bool f_gold ( char str [] ) {
  int n = strlen(str);
  if ( ( str [ n - 1 ] - '0' ) % 2 != 0 ) return false;
  int digitSum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) digitSum += ( str [ i ] - '0' );
  return ( digitSum % 3 == 0 );
}


bool f_filled ( char str [] ) {}

int main(void) {
    int n_success = 0;
    char param0[][100] = {"2112","1124","1110","O","65530186","132","UqOE","587","1010","QETUfLQ"};
    for(int i = 0; i < len(param0); ++i)
    {
        if(f_filled(param0[i]) == f_gold(param0[i]))
        {
            n_success+=1;
        }
	break;
	}
    printf("#Results:", " ", n_success, ", ", len(param0));
    return 0;
}