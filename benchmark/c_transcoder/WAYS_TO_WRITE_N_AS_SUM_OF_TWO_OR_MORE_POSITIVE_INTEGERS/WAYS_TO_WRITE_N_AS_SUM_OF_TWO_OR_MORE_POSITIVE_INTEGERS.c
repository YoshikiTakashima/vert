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
  int table [ n + 1 ];
  memset ( table, 0, sizeof ( table ) );
  table [ 0 ] = 1;
  for ( int i = 1;
  i < n;
  i ++ ) for ( int j = i;
  j <= n;
  j ++ ) table [ j ] += table [ j - i ];
  return table [ n ];
}


int f_filled ( int n ) {}

int main(void) {
    int n_success = 0;
    int param0[] = {66,90,8,77,44,20,34,22,50,10};
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