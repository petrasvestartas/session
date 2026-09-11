# Transformation - C++

```cpp
void ClassName::transform() {
    // Apply transformation
    double nx = xform[0]*_x + xform[1]*_y + xform[2]*_z + xform[3];
    double ny = xform[4]*_x + xform[5]*_y + xform[6]*_z + xform[7];
    double nz = xform[8]*_x + xform[9]*_y + xform[10]*_z + xform[11];

    _x = nx;
    _y = ny;
    _z = nz;

    // Reset to identity
    xform = Xform();
}

ClassName ClassName::transformed() const {
    ClassName result = *this;
    result.transform();
    return result;
}
```

## Using Xform::apply

```cpp
void ClassName::transform() {
    auto [nx, ny, nz] = xform.apply(_x, _y, _z);
    _x = nx;
    _y = ny;
    _z = nz;
    xform = Xform();
}
```
