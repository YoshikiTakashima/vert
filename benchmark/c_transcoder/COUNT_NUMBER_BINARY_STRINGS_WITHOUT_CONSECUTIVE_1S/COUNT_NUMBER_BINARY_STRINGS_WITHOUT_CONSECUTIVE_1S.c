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

int f_gold ( int n ) {
  int a [ n ], b [ n ];
  a [ 0 ] = b [ 0 ] = 1;
  for ( int i = 1;
  i < n;
  i ++ ) {
    a [ i ] = a [ i - 1 ] + b [ i - 1 ];
    b [ i ] = a [ i - 1 ];
  }
  return a [ n - 1 ] + b [ n - 1 ];
}


int f_filled ( int n ) {}

int main(void) {
    int n_success = 0;
    int param0[] = {86,75,14,5,41,35,30,89,84,53};
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