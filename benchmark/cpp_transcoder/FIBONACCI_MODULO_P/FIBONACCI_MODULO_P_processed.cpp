
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int p ) {
  int first = 1, second = 1, number = 2, next = 1;
  while ( next ) {
    next = ( first + second ) % p;
    first = second;
    second = next;
    number ++;
  }
  return number;
}


