
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


