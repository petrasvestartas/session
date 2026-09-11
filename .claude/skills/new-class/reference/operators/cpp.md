# Operators - C++

## Index Operator

```cpp
// Header:
double operator[](size_t i) const;
double& operator[](size_t i);

// Implementation:
double ClassName::operator[](size_t i) const {
    switch (i) {
        case 0: return _x;
        case 1: return _y;
        case 2: return _z;
        default: throw std::out_of_range("Index out of bounds");
    }
}

double& ClassName::operator[](size_t i) {
    switch (i) {
        case 0: return _x;
        case 1: return _y;
        case 2: return _z;
        default: throw std::out_of_range("Index out of bounds");
    }
}
```

## Equality Operators

```cpp
// Header:
bool operator==(const ClassName& other) const;
bool operator!=(const ClassName& other) const;

// Implementation:
bool ClassName::operator==(const ClassName& other) const {
    return _x == other._x && _y == other._y && _z == other._z;
    // Note: guid is NOT compared
}

bool ClassName::operator!=(const ClassName& other) const {
    return !(*this == other);
}
```

## Arithmetic Operators

```cpp
// Header:
ClassName& operator+=(const Vector& v);
ClassName& operator-=(const Vector& v);
ClassName& operator*=(double scalar);
ClassName& operator/=(double scalar);
ClassName operator+(const Vector& v) const;
ClassName operator-(const Vector& v) const;
ClassName operator*(double scalar) const;
ClassName operator/(double scalar) const;

// Implementation:
ClassName& ClassName::operator+=(const Vector& v) {
    _x += v[0]; _y += v[1]; _z += v[2];
    return *this;
}

ClassName ClassName::operator+(const Vector& v) const {
    ClassName result = *this;
    result += v;
    return result;
}

ClassName& ClassName::operator*=(double scalar) {
    _x *= scalar; _y *= scalar; _z *= scalar;
    return *this;
}

ClassName ClassName::operator*(double scalar) const {
    ClassName result = *this;
    result *= scalar;
    return result;
}
```
