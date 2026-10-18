# Spec Delta

## Purpose

Provides the vector and matrix arithmetic that calibration, camera pose estimation and drawings rely on, with single-precision storage like the original and well-defined behaviour for degenerate inputs.

## ADDED Requirements

### Requirement: 2D and 3D vectors
The system SHALL provide a 2D vector (from components or from two points) with add, subtract, scale, divide, dot product, norm, squared norm, normalization and negation, and a 3D vector with component-wise add, subtract, multiply, scalar operations, dot and cross products, norm, min/max component and index, and conversion to an array.

#### Scenario: Vector from two points
- **WHEN** a 2D vector is built from (1, 2) to (4, 6)
- **THEN** its components are (3, 4), its norm is 5 and its normalized form is (0.6, 0.8)

#### Scenario: Normalizing the zero vector
- **WHEN** the zero vector is normalized
- **THEN** the result is NaN components, matching the original division, and is documented as such

### Requirement: 3×3 matrix operations
The system SHALL provide a 3×3 single-precision matrix with identity, determinant, rotations about X, Y and Z, construction from yaw/pitch/roll and extraction back, construction from rows, columns and diagonal, row and column access, multiplication, transpose, inverse and multiplication by a 3D vector.

#### Scenario: Yaw pitch roll round-trip
- **WHEN** a matrix is created from yaw 0.3, pitch −0.2, roll 0.5 radians and the angles are extracted
- **THEN** each extracted angle equals the input within 1e-5

#### Scenario: Inverse of a rotation
- **WHEN** a rotation matrix is multiplied by its inverse
- **THEN** the result equals identity within 1e-5 per element

### Requirement: Singular value decomposition
The system SHALL decompose any m×n matrix with m ≥ n into U, W, V such that A = U·diag(W)·Vᵀ, with the same convergence behaviour as the original routine (at most 30 iterations, error when exceeded).

#### Scenario: Reconstruction
- **WHEN** a random 5×3 matrix with fixed seed is decomposed
- **THEN** U·diag(W)·Vᵀ reconstructs the input within 1e-9 and the columns of U and V are orthonormal within 1e-9

#### Scenario: Oracle parity
- **WHEN** the matrices in `tests/fixtures/geometry/svd/` are decomposed
- **THEN** the singular values equal the fixture within 1e-9 (sign and order of singular vectors are compared up to sign)
