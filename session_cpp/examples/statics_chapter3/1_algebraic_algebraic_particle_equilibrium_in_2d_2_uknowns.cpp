#include "core.hpp"

int main() {

  // Problem initialization. 
  // Vectors are defined from scalars, since the length of the vectors is not known yet.
  geo::Vector Ta = geo::Vector::from_scalars(-0.8, 0.6);
  geo::Vector Tc = geo::Vector::from_scalars(cos(45*geo::GLOBALS::TO_RADIANS), sin(45*geo::GLOBALS::TO_RADIANS));
  geo::Vector Tbd(0, -588,6);
  geo::log(Ta.to_string());
  geo::log(Tc.to_string());
  geo::log(Tbd.to_string());


  // Express Ta vector by Tc, x coordinate in Tbd is 0
  int a = Tbd[0] == 0 ? 0 : 1;
  int b = abs(a-1);
  double Ta_by_Tc = -Tc(a) / Ta(a);

  // Solve the equations
  // When the length function gets an input, xyz coordinates are recalculated using abc unit scalars.
  Tc.length( -Tbd[1] / (Tc(b)+Ta(b)*Ta_by_Tc) );
  Ta.length( Ta_by_Tc*Tc.length() );
  geo::log(Ta.to_string());
  geo::log(Tc.to_string());

  return 1;
}