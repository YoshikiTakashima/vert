
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int a [ ], int n, int k ) {
  if ( k >= n - 1 ) return n;
  int best = 0, times = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( a [ i ] > best ) {
      best = a [ i ];
      if ( i ) times = 1;
    }
    else times += 1;
    if ( times >= k ) return best;
  }
  return best;
}


