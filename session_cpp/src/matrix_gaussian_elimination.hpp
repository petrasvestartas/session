#pragma once

#include "matrix.hpp"
#include <algorithm>
#include <cassert>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <vector>

namespace geo {

void swap_rows(Matrix& m, size_t i, size_t j) {
    size_t columns = m.get_cols();
    for (size_t k = 0; k < columns; ++k)
        std::swap(m(i, k), m(j, k));
}

std::vector<double> gauss_partial(const Matrix& a0, const std::vector<double>& b0) {
    size_t n = a0.get_rows();
    assert(a0.get_cols() == n);
    assert(b0.size() == n);
    
    // make augmented matrix
    Matrix a(n, n + 1);
    for (size_t i = 0; i < n; ++i) {
        for (size_t j = 0; j < n; ++j)
            a(i, j) = a0(i, j);
        a(i, n) = b0[i];
    }
    
    // WP algorithm from Gaussian elimination page
    // produces row echelon form
    for (size_t k = 0; k < n; ++k) {
        // Find pivot for column k
        size_t max_index = k;
        double max_value = 0;
        for (size_t i = k; i < n; ++i) {
            // compute scale factor = max abs in row
            double scale_factor = 0;
            for (size_t j = k; j < n; ++j)
                scale_factor = std::max(std::abs(a(i, j)), scale_factor);
            if (scale_factor == 0)
                continue;
            // scale the abs used to pick the pivot
            double abs = std::abs(a(i, k))/scale_factor;
            if (abs > max_value) {
                max_index = i;
                max_value = abs;
            }
        }
        if (a(max_index, k) == 0)
            throw std::runtime_error("matrix is singular");
        if (k != max_index)
            swap_rows(a, k, max_index);
        for (size_t i = k + 1; i < n; ++i) {
            double f = a(i, k)/a(k, k);
            for (size_t j = k + 1; j <= n; ++j)
                a(i, j) -= a(k, j) * f;
            a(i, k) = 0;
        }
    }
    
    // now back substitute to get result
    std::vector<double> x(n);
    for (size_t i = n; i-- > 0; ) {
        x[i] = a(i, n);
        for (size_t j = i + 1; j < n; ++j)
            x[i] -= a(i, j) * x[j];
        x[i] /= a(i, i);
    }
    return x;
}

} // namespace geo