
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int A [ ], int N, int M ) {
  int sum = 0;
  int ans = 0;
  for ( int i = 0;
  i < N;
  i ++ ) {
    for ( int j = i + 1;
    j < N;
    j ++ ) {
      for ( int k = j + 1;
      k < N;
      k ++ ) {
        sum = A [ i ] + A [ j ] + A [ k ];
        if ( sum % M == 0 ) ans ++;
      }
    }
  }
  return ans;
}


