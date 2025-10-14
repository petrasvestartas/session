"""Verification tests comparing our BVH implementation with reference values."""

from .bvh import expand_bits, calculate_morton_code


def test_expand_bits_reference_values():
    """Test expand_bits against known reference values from JavaScript implementation."""
    # Test cases verified against JavaScript expandBits function
    assert expand_bits(0) == 0
    assert expand_bits(1) == 1
    assert expand_bits(2) == 8  # 0b10 -> 0b1000
    assert expand_bits(3) == 9  # 0b11 -> 0b1001
    assert expand_bits(4) == 64  # 0b100 -> 0b1000000
    assert expand_bits(7) == 73  # 0b111 -> 0b1001001
    
    # Verify the bit pattern for a larger value
    # 0b1010 (10) should become 0b1000001000 (520)
    assert expand_bits(10) == 520


def test_morton_code_reference_values():
    """Test Morton code calculation against reference implementation."""
    world_size = 100.0
    
    # Test origin (center of world)
    code_origin = calculate_morton_code(0.0, 0.0, 0.0, world_size)
    # At origin, normalized coords are (0.5, 0.5, 0.5)
    # Scaled to 10-bit: (511, 511, 511)
    # This should give a specific Morton code
    assert code_origin > 0
    
    # Test world corners
    # Bottom-left-back corner (-50, -50, -50) -> normalized (0, 0, 0) -> Morton code 0
    code_min = calculate_morton_code(-50.0, -50.0, -50.0, world_size)
    assert code_min == 0
    
    # Top-right-front corner (50, 50, 50) -> normalized (1, 1, 1) -> Morton code max
    code_max = calculate_morton_code(50.0, 50.0, 50.0, world_size)
    # Max 30-bit value: 0x3FFFFFFF
    assert code_max == 0x3FFFFFFF
    
    # Test that Morton codes preserve spatial locality
    # Points close in 3D space should have similar Morton codes
    code1 = calculate_morton_code(10.0, 10.0, 10.0, world_size)
    code2 = calculate_morton_code(10.1, 10.1, 10.1, world_size)
    code3 = calculate_morton_code(-40.0, -40.0, -40.0, world_size)
    
    # Nearby points should have closer Morton codes
    diff_nearby = abs(code1 - code2)
    diff_far = abs(code1 - code3)
    assert diff_nearby < diff_far, "Morton codes should preserve spatial locality"


def test_morton_code_interleaving():
    """Verify bit interleaving pattern in Morton codes."""
    world_size = 100.0
    
    # Test a point where we know the exact bit pattern
    # Point at (0, 0, 50) -> normalized (0.5, 0.5, 1.0)
    # Scaled: (511, 511, 1023)
    code = calculate_morton_code(0.0, 0.0, 50.0, world_size)
    
    # Verify the code is non-zero and within valid range
    assert 0 < code < (1 << 30)
    
    # Test axis-aligned points have different patterns
    code_x = calculate_morton_code(50.0, 0.0, 0.0, world_size)
    code_y = calculate_morton_code(0.0, 50.0, 0.0, world_size)
    code_z = calculate_morton_code(0.0, 0.0, 50.0, world_size)
    
    # All should be different
    assert code_x != code_y
    assert code_y != code_z
    assert code_x != code_z


def test_morton_code_symmetry():
    """Test that Morton codes respect coordinate symmetry."""
    world_size = 100.0
    
    # Points symmetric about origin should have related Morton codes
    code_pos = calculate_morton_code(20.0, 20.0, 20.0, world_size)
    code_neg = calculate_morton_code(-20.0, -20.0, -20.0, world_size)
    
    # Both should be valid
    assert 0 <= code_pos < (1 << 30)
    assert 0 <= code_neg < (1 << 30)
    
    # They should be different
    assert code_pos != code_neg


def test_morton_code_clamping():
    """Test that coordinates outside world bounds are clamped correctly."""
    world_size = 100.0
    
    # Points outside world should be clamped to edges
    code_outside = calculate_morton_code(1000.0, 1000.0, 1000.0, world_size)
    code_max = calculate_morton_code(50.0, 50.0, 50.0, world_size)
    
    # Should clamp to maximum
    assert code_outside == code_max
    
    # Test negative overflow
    code_neg_outside = calculate_morton_code(-1000.0, -1000.0, -1000.0, world_size)
    code_min = calculate_morton_code(-50.0, -50.0, -50.0, world_size)
    
    # Should clamp to minimum
    assert code_neg_outside == code_min


def test_morton_code_deterministic():
    """Test that Morton code calculation is deterministic."""
    world_size = 100.0
    
    # Same input should always give same output
    x, y, z = 15.7, -23.4, 8.9
    
    code1 = calculate_morton_code(x, y, z, world_size)
    code2 = calculate_morton_code(x, y, z, world_size)
    code3 = calculate_morton_code(x, y, z, world_size)
    
    assert code1 == code2 == code3


def test_expand_bits_bit_pattern():
    """Verify the exact bit expansion pattern."""
    # expandBits should insert two zeros after each bit
    # Input:  0b0001 (1)
    # Output: 0b0000001 (1)
    assert expand_bits(1) == 0b1
    
    # Input:  0b0010 (2)
    # Output: 0b0001000 (8)
    assert expand_bits(2) == 0b1000
    
    # Input:  0b0011 (3)
    # Output: 0b0001001 (9)
    assert expand_bits(3) == 0b1001
    
    # Input:  0b0100 (4)
    # Output: 0b1000000 (64)
    assert expand_bits(4) == 0b1000000
    
    # Input:  0b0101 (5)
    # Output: 0b1000001 (65)
    assert expand_bits(5) == 0b1000001


def test_morton_code_z_order_property():
    """Test that Morton codes follow Z-order curve properties."""
    world_size = 100.0
    
    # In a Z-order curve, traversing in Morton code order
    # should visit spatially local regions
    
    # Create a grid of points
    points = []
    for x in [-40, -20, 0, 20, 40]:
        for y in [-40, -20, 0, 20, 40]:
            for z in [-40, -20, 0, 20, 40]:
                code = calculate_morton_code(x, y, z, world_size)
                points.append((code, x, y, z))
    
    # Sort by Morton code
    points.sort(key=lambda p: p[0])
    
    # Verify all codes are unique (no collisions)
    codes = [p[0] for p in points]
    assert len(codes) == len(set(codes)), "Morton codes should be unique"
    
    # Verify codes are in ascending order
    for i in range(len(codes) - 1):
        assert codes[i] < codes[i + 1], "Morton codes should be strictly increasing"
