#pragma once

#include <iostream>
#include <sstream>
#include <vector>

namespace geo {

class Matrix {
   public:
    // Constructor to initialize an m x n matrix with zeros
    Matrix(size_t rows, size_t cols);

    // Constructor with initializer list
    Matrix(std::initializer_list<std::initializer_list<double>> values);

    // Access elements
    double& operator()(size_t row, size_t col);
    const double& operator()(size_t row, size_t col) const;

    // Matrix multiplication
    Matrix operator*(const Matrix& other) const;

    // Convert matrix to string
    std::string to_string() const;

    // Get number of rows
    size_t get_rows() const;

    // Get number of columns
    size_t get_cols() const;

    // Static method to create an identity matrix
    static Matrix identity(size_t size);

   private:
    size_t rows;
    size_t cols;
    std::vector<std::vector<double>> data;
};
}  // namespace geo