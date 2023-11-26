
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int process, int need ) {
  int minResources = 0;
  minResources = process * ( need - 1 ) + 1;
  return minResources;
}


