// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//

#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  return ( 3 * n * n - n ) / 2;
}


//TOFILL

int f_filled ( int n ) {}
int main() {
    int n_success = 0;
    vector<int> param0 {96,93,15,8,21,14,11,79,24,94};
    for(int i = 0; i < param0.size(); ++i)
    {
        if(f_filled(param0[i]) == f_gold(param0[i]))
        {
            n_success+=1;
        }
	break;
	}
    cout << "#Results:" << " " << n_success << ", " << param0.size();
    return 0;
}