#include <cassert>
#include <numeric>


#include "functions.h"

double sum(ACollection a, BCollection b) {
  assert(a.size == b.size);
  double result = 0.0;
  result = std::accumulate(a.data, a.data + a.size, result,
                           [](double left, const auto &right) {
                             return left + right.first + right.second;
                           });
  result = std::accumulate(b.data, b.data + b.size, result,
                           [](double left, const auto &right) {
                             return left + right.first + right.second;
                           });
  return result;
}

ACollection allocACollection(int size) {
  return {size == 0 ? nullptr : new A[size], size};
}
BCollection allocBCollection(int size) {
  return {size == 0 ? nullptr : new B[size], size};
}

void freeACollection(ACollection collection) { delete[] collection.data; }
void freeBCollection(BCollection collection) { delete[] collection.data; }