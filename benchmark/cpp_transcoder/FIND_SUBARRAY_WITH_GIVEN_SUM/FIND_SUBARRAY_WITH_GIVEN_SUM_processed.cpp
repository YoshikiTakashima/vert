
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n, int sum ) {
  int curr_sum, i, j;
  for ( i = 0;
  i < n;
  i ++ ) {
    curr_sum = arr [ i ];
    for ( j = i + 1;
    j <= n;
    j ++ ) {
      if ( curr_sum == sum ) {
        cout << "Sum found between indexes " << i << " and " << j - 1;
        return 1;
      }
      if ( curr_sum > sum || j == n ) break;
      curr_sum = curr_sum + arr [ j ];
    }
  }
  cout << "No subarray found";
  return 0;
}


