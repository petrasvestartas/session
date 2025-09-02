#include "matrix.hpp"

#include <algorithm>  // Include algorithm for std::max_element
#include <iomanip>  // Include the iomanip header for std::fixed, std::setprecision, std::setw, and std::showpos
#include <limits>  // Include limits for std::numeric_limits

namespace geo {

// Constructor to initialize an m x n matrix with zeros
Matrix::Matrix(size_t rows, size_t cols)
    : rows(rows), cols(cols), data(rows, std::vector<double>(cols, 0.0)) {}

// Constructor with initializer list
Matrix::Matrix(std::initializer_list<std::initializer_list<double>> values) {
    rows = values.size();
    cols = values.begin()->size();
    data.resize(rows);
    size_t row = 0;
    for (auto& row_values : values) {
        data[row].resize(row_values.size());
        size_t col = 0;
        for (auto& value : row_values) {
            data[row][col] = value;
            ++col;
        }
        ++row;
    }
}

// Access elements
double& Matrix::operator()(size_t row, size_t col) { return data[row][col]; }

const double& Matrix::operator()(size_t row, size_t col) const { return data[row][col]; }

// Matrix multiplication
Matrix Matrix::operator*(const Matrix& other) const {
    if (cols != other.rows) {
        throw std::invalid_argument("Matrix sizes do not match for multiplication");
    }

    Matrix result(rows, other.cols);
    for (size_t i = 0; i < rows; ++i) {
        for (size_t j = 0; j < other.cols; ++j) {
            for (size_t k = 0; k < cols; ++k) {
                result(i, j) += (*this)(i, k) * other(k, j);
            }
        }
    }
    return result;
}

// Convert matrix to string
std::string Matrix::to_string() const {
    std::ostringstream oss;
    oss << "Matrix: \n";
    oss << std::fixed
        << std::setprecision(
               std::numeric_limits<double>::max_digits10);  // Set fixed-point notation, max
                                                            // precision, and show positive sign

    // Print the matrix
    for (const auto& row : data) {
        for (const auto& value : row) {
            oss << value << " ";
        }
        oss << "\n";
    }

    return oss.str();
}

// Get number of rows
size_t Matrix::get_rows() const { return rows; }

// Get number of columns
size_t Matrix::get_cols() const { return cols; }

// Static method to create an identity matrix
Matrix Matrix::identity(size_t size) {
    Matrix result(size, size);
    for (size_t i = 0; i < size; ++i) {
        result(i, i) = 1.0;
    }
    return result;
}
}  // namespace geo