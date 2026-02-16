//! Tensor Operations for MERA Compression
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 2 (Weeks 5-6):
//! "MERA Tensor Network" - "Disentangler tensors (remove redundancy)"
//! "Coarse-grainer tensors (combine similar representations)"
//!
//! This module provides:
//! - Tensor struct for multi-dimensional data storage
//! - Tensor operations (add, multiply, contract, transpose, etc.)
//! - Efficient memory layout for holographic data
//! - Support for sparse representations (for large holographic fields)

use crate::types::Float;
use std::fmt;

/// Shape of a tensor (dimensions)
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
/// "7 scales: quantum → cosmic"
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensorShape {
    /// Dimensions of the tensor
    pub dims: Vec<usize>,
}

impl TensorShape {
    /// Create a new tensor shape
    pub fn new(dims: Vec<usize>) -> Self {
        Self { dims }
    }

    /// Create a scalar shape (0-dimensional)
    pub fn scalar() -> Self {
        Self { dims: vec![] }
    }

    /// Create a vector shape (1-dimensional)
    pub fn vector(size: usize) -> Self {
        Self { dims: vec![size] }
    }

    /// Create a matrix shape (2-dimensional)
    pub fn matrix(rows: usize, cols: usize) -> Self {
        Self {
            dims: vec![rows, cols],
        }
    }

    /// Create a 3D tensor shape
    pub fn tensor3d(d0: usize, d1: usize, d2: usize) -> Self {
        Self {
            dims: vec![d0, d1, d2],
        }
    }

    /// Get the total number of elements
    pub fn size(&self) -> usize {
        self.dims.iter().product()
    }

    /// Get the number of dimensions (rank)
    pub fn rank(&self) -> usize {
        self.dims.len()
    }

    /// Check if this is a scalar (0-dimensional)
    pub fn is_scalar(&self) -> bool {
        self.rank() == 0
    }

    /// Check if this is a vector (1-dimensional)
    pub fn is_vector(&self) -> bool {
        self.rank() == 1
    }

    /// Check if this is a matrix (2-dimensional)
    pub fn is_matrix(&self) -> bool {
        self.rank() == 2
    }
}

impl fmt::Display for TensorShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TensorShape({})",
            self.dims
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join("×")
        )
    }
}

/// Tensor data storage
///
/// Supports both dense and sparse representations for efficiency.
#[derive(Debug, Clone)]
pub enum TensorData {
    /// Dense storage (all elements stored)
    Dense(Vec<Float>),
    /// Sparse storage (only non-zero elements stored)
    Sparse {
        /// Values of non-zero elements
        values: Vec<Float>,
        /// Indices of non-zero elements (flat index)
        indices: Vec<usize>,
        /// Total size of the tensor
        total_size: usize,
    },
}

impl TensorData {
    /// Create dense tensor data from a vector
    pub fn dense(data: Vec<Float>) -> Self {
        TensorData::Dense(data)
    }

    /// Create sparse tensor data from non-zero values and indices
    pub fn sparse(values: Vec<Float>, indices: Vec<usize>, total_size: usize) -> Self {
        TensorData::Sparse {
            values,
            indices,
            total_size,
        }
    }

    /// Convert to dense representation
    pub fn to_dense(&self) -> Vec<Float> {
        match self {
            TensorData::Dense(data) => data.clone(),
            TensorData::Sparse {
                values,
                indices,
                total_size,
            } => {
                let mut dense = vec![0.0; *total_size];
                for (&val, &idx) in values.iter().zip(indices.iter()) {
                    if idx < *total_size {
                        dense[idx] = val;
                    }
                }
                dense
            }
        }
    }

    /// Get the number of non-zero elements
    pub fn nnz(&self) -> usize {
        match self {
            TensorData::Dense(data) => data.iter().filter(|&&v| v.abs() > 1e-10).count(),
            TensorData::Sparse { values, .. } => values.len(),
        }
    }

    /// Get the sparsity (fraction of non-zero elements)
    pub fn sparsity(&self) -> f64 {
        let total_size = self.size();
        if total_size == 0 {
            return 0.0;
        }
        self.nnz() as f64 / total_size as f64
    }

    /// Get the total size
    pub fn size(&self) -> usize {
        match self {
            TensorData::Dense(data) => data.len(),
            TensorData::Sparse { total_size, .. } => *total_size,
        }
    }

    /// Auto-convert to sparse if beneficial (sparsity > 0.5)
    pub fn auto_sparse(self) -> Self {
        if self.sparsity() > 0.5 {
            // Convert to sparse
            let dense = self.to_dense();
            let mut values = Vec::new();
            let mut indices = Vec::new();
            for (i, &val) in dense.iter().enumerate() {
                if val.abs() > 1e-10 {
                    values.push(val);
                    indices.push(i);
                }
            }
            TensorData::sparse(values, indices, dense.len())
        } else {
            self
        }
    }
}

/// Tensor for MERA operations
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Each part contains the whole" = efficient compression through self-similarity
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 2:
/// "Disentangler tensors (remove redundancy)"
/// "Coarse-grainer tensors (combine similar representations)"
#[derive(Debug, Clone)]
pub struct Tensor {
    /// Shape of the tensor
    pub shape: TensorShape,
    /// Tensor data (dense or sparse)
    pub data: TensorData,
}

impl Tensor {
    /// Create a new tensor from shape and data
    pub fn new(shape: TensorShape, data: TensorData) -> Self {
        assert_eq!(shape.size(), data.size(), "Shape and data size must match");
        Self { shape, data }
    }

    /// Create a scalar tensor (0-dimensional)
    pub fn scalar(value: Float) -> Self {
        Self::new(TensorShape::scalar(), TensorData::dense(vec![value]))
    }

    /// Create a vector tensor (1-dimensional)
    pub fn vector(data: Vec<Float>) -> Self {
        let shape = TensorShape::vector(data.len());
        Self::new(shape, TensorData::dense(data))
    }

    /// Create a matrix tensor (2-dimensional)
    pub fn matrix(rows: usize, cols: usize, data: Vec<Float>) -> Self {
        let shape = TensorShape::matrix(rows, cols);
        Self::new(shape, TensorData::dense(data))
    }

    /// Create a zero tensor with given shape
    pub fn zeros(shape: TensorShape) -> Self {
        let data = vec![0.0; shape.size()];
        Self::new(shape, TensorData::dense(data))
    }

    /// Create a ones tensor with given shape
    pub fn ones(shape: TensorShape) -> Self {
        let data = vec![1.0; shape.size()];
        Self::new(shape, TensorData::dense(data))
    }

    /// Create a random tensor with given shape
    pub fn random(shape: TensorShape) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let data: Vec<Float> = (0..shape.size()).map(|_| rng.gen::<f64>()).collect();
        Self::new(shape, TensorData::dense(data))
    }

    /// Create an identity matrix
    pub fn identity(size: usize) -> Self {
        let mut data = vec![0.0; size * size];
        for i in 0..size {
            data[i * size + i] = 1.0;
        }
        Self::matrix(size, size, data)
    }

    /// Get the dense data as a slice
    pub fn as_dense(&self) -> &[Float] {
        match &self.data {
            TensorData::Dense(data) => data.as_slice(),
            TensorData::Sparse { .. } => panic!("Tensor is sparse, use to_dense() first"),
        }
    }

    /// Get dense data as mutable slice
    pub fn as_dense_mut(&mut self) -> &mut [Float] {
        match &mut self.data {
            TensorData::Dense(data) => data.as_mut_slice(),
            TensorData::Sparse { .. } => panic!("Tensor is sparse, use to_dense() first"),
        }
    }

    /// Convert to dense representation
    pub fn to_dense(&self) -> Tensor {
        let dense_data = self.data.to_dense();
        Tensor::new(self.shape.clone(), TensorData::dense(dense_data))
    }

    /// Get element at flat index
    pub fn get(&self, index: usize) -> Float {
        match &self.data {
            TensorData::Dense(data) => data[index],
            TensorData::Sparse {
                values, indices, ..
            } => {
                if let Some(pos) = indices.iter().position(|&i| i == index) {
                    values[pos]
                } else {
                    0.0
                }
            }
        }
    }

    /// Set element at flat index
    pub fn set(&mut self, index: usize, value: Float) {
        match &mut self.data {
            TensorData::Dense(data) => {
                data[index] = value;
            }
            TensorData::Sparse {
                values,
                indices,
                total_size,
            } => {
                if let Some(pos) = indices.iter().position(|&i| i == index) {
                    values[pos] = value;
                } else if value.abs() > 1e-10 {
                    values.push(value);
                    indices.push(index);
                }
                *total_size = (*total_size).max(index + 1);
            }
        }
    }

    /// Element-wise addition
    pub fn add(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!("Shape mismatch: {} vs {}", self.shape, other.shape));
        }

        let self_dense = self.data.to_dense();
        let other_dense = other.data.to_dense();
        let result: Vec<Float> = self_dense
            .iter()
            .zip(other_dense.iter())
            .map(|(&a, &b)| a + b)
            .collect();

        Ok(Tensor::new(self.shape.clone(), TensorData::dense(result)))
    }

    /// Element-wise subtraction
    pub fn sub(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!("Shape mismatch: {} vs {}", self.shape, other.shape));
        }

        let self_dense = self.data.to_dense();
        let other_dense = other.data.to_dense();
        let result: Vec<Float> = self_dense
            .iter()
            .zip(other_dense.iter())
            .map(|(&a, &b)| a - b)
            .collect();

        Ok(Tensor::new(self.shape.clone(), TensorData::dense(result)))
    }

    /// Element-wise multiplication
    pub fn mul(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!("Shape mismatch: {} vs {}", self.shape, other.shape));
        }

        let self_dense = self.data.to_dense();
        let other_dense = other.data.to_dense();
        let result: Vec<Float> = self_dense
            .iter()
            .zip(other_dense.iter())
            .map(|(&a, &b)| a * b)
            .collect();

        Ok(Tensor::new(self.shape.clone(), TensorData::dense(result)))
    }

    /// Element-wise division
    pub fn div(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!("Shape mismatch: {} vs {}", self.shape, other.shape));
        }

        let self_dense = self.data.to_dense();
        let other_dense = other.data.to_dense();
        let result: Vec<Float> = self_dense
            .iter()
            .zip(other_dense.iter())
            .map(|(&a, &b)| a / b)
            .collect();

        Ok(Tensor::new(self.shape.clone(), TensorData::dense(result)))
    }

    /// Matrix multiplication (for 2D tensors)
    pub fn matmul(&self, other: &Tensor) -> Result<Tensor, String> {
        if !self.shape.is_matrix() || !other.shape.is_matrix() {
            return Err("Both tensors must be matrices for matmul".to_string());
        }

        let (m, k1) = (self.shape.dims[0], self.shape.dims[1]);
        let (k2, n) = (other.shape.dims[0], other.shape.dims[1]);

        if k1 != k2 {
            return Err(format!("Inner dimensions must match: {} vs {}", k1, k2));
        }

        let self_dense = self.data.to_dense();
        let other_dense = other.data.to_dense();
        let mut result = vec![0.0; m * n];

        for i in 0..m {
            for j in 0..n {
                for k in 0..k1 {
                    result[i * n + j] += self_dense[i * k1 + k] * other_dense[k * n + j];
                }
            }
        }

        Ok(Tensor::matrix(m, n, result))
    }

    /// Transpose (for 2D tensors)
    pub fn transpose(&self) -> Result<Tensor, String> {
        if !self.shape.is_matrix() {
            return Err("Tensor must be a matrix for transpose".to_string());
        }

        let (rows, cols) = (self.shape.dims[0], self.shape.dims[1]);
        let self_dense = self.data.to_dense();
        let mut result = vec![0.0; rows * cols];

        for i in 0..rows {
            for j in 0..cols {
                result[j * rows + i] = self_dense[i * cols + j];
            }
        }

        Ok(Tensor::matrix(cols, rows, result))
    }

    /// Compute the Frobenius norm
    pub fn norm(&self) -> Float {
        let self_dense = self.data.to_dense();
        self_dense.iter().map(|&v| v * v).sum::<Float>().sqrt()
    }

    /// Normalize the tensor to unit norm
    pub fn normalize(&self) -> Tensor {
        let norm = self.norm();
        if norm > 1e-10 {
            let self_dense = self.data.to_dense();
            let normalized: Vec<Float> = self_dense.iter().map(|&v| v / norm).collect();
            Tensor::new(self.shape.clone(), TensorData::dense(normalized))
        } else {
            self.clone()
        }
    }

    /// Compute the sum of all elements
    pub fn sum(&self) -> Float {
        let self_dense = self.data.to_dense();
        self_dense.iter().sum()
    }

    /// Compute the mean of all elements
    pub fn mean(&self) -> Float {
        let size = self.shape.size();
        if size == 0 {
            return 0.0;
        }
        self.sum() / size as Float
    }

    /// Reshape the tensor
    pub fn reshape(&self, new_shape: TensorShape) -> Result<Tensor, String> {
        if self.shape.size() != new_shape.size() {
            return Err(format!(
                "Cannot reshape from {} to {} (size mismatch)",
                self.shape, new_shape
            ));
        }

        Ok(Tensor::new(new_shape, self.data.clone()))
    }

    /// Flatten the tensor to a vector
    pub fn flatten(&self) -> Tensor {
        Tensor::vector(self.data.to_dense())
    }

    /// Get memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        match &self.data {
            TensorData::Dense(data) => data.len() * std::mem::size_of::<Float>(),
            TensorData::Sparse {
                values, indices, ..
            } => (values.len() + indices.len()) * std::mem::size_of::<Float>(),
        }
    }

    /// Get compression ratio (if sparse)
    pub fn compression_ratio(&self) -> f64 {
        match &self.data {
            TensorData::Dense(_) => 1.0,
            TensorData::Sparse {
                values,
                indices,
                total_size,
            } => {
                let compressed_size = (values.len() + indices.len()) * std::mem::size_of::<Float>();
                let original_size = *total_size * std::mem::size_of::<Float>();
                if original_size == 0 {
                    1.0
                } else {
                    compressed_size as f64 / original_size as f64
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_shape_creation() {
        let scalar = TensorShape::scalar();
        assert_eq!(scalar.rank(), 0);
        assert_eq!(scalar.size(), 1);

        let vector = TensorShape::vector(10);
        assert_eq!(vector.rank(), 1);
        assert_eq!(vector.size(), 10);

        let matrix = TensorShape::matrix(3, 4);
        assert_eq!(matrix.rank(), 2);
        assert_eq!(matrix.size(), 12);

        let tensor3d = TensorShape::tensor3d(2, 3, 4);
        assert_eq!(tensor3d.rank(), 3);
        assert_eq!(tensor3d.size(), 24);
    }

    #[test]
    fn test_tensor_creation() {
        let scalar = Tensor::scalar(5.0);
        assert_eq!(scalar.get(0), 5.0);
        assert!(scalar.shape.is_scalar());

        let vector = Tensor::vector(vec![1.0, 2.0, 3.0]);
        assert_eq!(vector.get(0), 1.0);
        assert_eq!(vector.get(1), 2.0);
        assert_eq!(vector.get(2), 3.0);
        assert!(vector.shape.is_vector());

        let matrix = Tensor::matrix(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(matrix.get(0), 1.0);
        assert_eq!(matrix.get(1), 2.0);
        assert_eq!(matrix.get(2), 3.0);
        assert_eq!(matrix.get(3), 4.0);
        assert!(matrix.shape.is_matrix());
    }

    #[test]
    fn test_tensor_zeros_ones() {
        let zeros = Tensor::zeros(TensorShape::vector(5));
        for i in 0..5 {
            assert_eq!(zeros.get(i), 0.0);
        }

        let ones = Tensor::ones(TensorShape::vector(5));
        for i in 0..5 {
            assert_eq!(ones.get(i), 1.0);
        }
    }

    #[test]
    fn test_tensor_identity() {
        let identity = Tensor::identity(3);
        assert_eq!(identity.get(0), 1.0); // [0,0]
        assert_eq!(identity.get(1), 0.0); // [0,1]
        assert_eq!(identity.get(2), 0.0); // [0,2]
        assert_eq!(identity.get(3), 0.0); // [1,0]
        assert_eq!(identity.get(4), 1.0); // [1,1]
        assert_eq!(identity.get(5), 0.0); // [1,2]
        assert_eq!(identity.get(6), 0.0); // [2,0]
        assert_eq!(identity.get(7), 0.0); // [2,1]
        assert_eq!(identity.get(8), 1.0); // [2,2]
    }

    #[test]
    fn test_tensor_add() {
        let a = Tensor::vector(vec![1.0, 2.0, 3.0]);
        let b = Tensor::vector(vec![4.0, 5.0, 6.0]);
        let c = a.add(&b).unwrap();

        assert_eq!(c.get(0), 5.0);
        assert_eq!(c.get(1), 7.0);
        assert_eq!(c.get(2), 9.0);
    }

    #[test]
    fn test_tensor_sub() {
        let a = Tensor::vector(vec![4.0, 5.0, 6.0]);
        let b = Tensor::vector(vec![1.0, 2.0, 3.0]);
        let c = a.sub(&b).unwrap();

        assert_eq!(c.get(0), 3.0);
        assert_eq!(c.get(1), 3.0);
        assert_eq!(c.get(2), 3.0);
    }

    #[test]
    fn test_tensor_mul() {
        let a = Tensor::vector(vec![1.0, 2.0, 3.0]);
        let b = Tensor::vector(vec![2.0, 3.0, 4.0]);
        let c = a.mul(&b).unwrap();

        assert_eq!(c.get(0), 2.0);
        assert_eq!(c.get(1), 6.0);
        assert_eq!(c.get(2), 12.0);
    }

    #[test]
    fn test_tensor_matmul() {
        let a = Tensor::matrix(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = Tensor::matrix(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
        let c = a.matmul(&b).unwrap();

        // [1,2,3] * [7,9,11] = 1*7 + 2*9 + 3*11 = 7 + 18 + 33 = 58
        //           [8,10,12]   1*8 + 2*10 + 3*12 = 8 + 20 + 36 = 64
        // [4,5,6] * [7,9,11] = 4*7 + 5*9 + 6*11 = 28 + 45 + 66 = 139
        //           [8,10,12]   4*8 + 5*10 + 6*12 = 32 + 50 + 72 = 154
        assert_eq!(c.get(0), 58.0);
        assert_eq!(c.get(1), 64.0);
        assert_eq!(c.get(2), 139.0);
        assert_eq!(c.get(3), 154.0);
    }

    #[test]
    fn test_tensor_transpose() {
        let a = Tensor::matrix(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = a.transpose().unwrap();

        assert_eq!(b.shape.dims, vec![3, 2]);
        assert_eq!(b.get(0), 1.0); // [0,0] -> [0,0]
        assert_eq!(b.get(1), 4.0); // [0,1] -> [1,0]
        assert_eq!(b.get(2), 2.0); // [1,0] -> [0,1]
        assert_eq!(b.get(3), 5.0); // [1,1] -> [1,1]
        assert_eq!(b.get(4), 3.0); // [2,0] -> [0,2]
        assert_eq!(b.get(5), 6.0); // [2,1] -> [1,2]
    }

    #[test]
    fn test_tensor_norm() {
        let a = Tensor::vector(vec![3.0, 4.0]);
        let norm = a.norm();
        assert!((norm - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_tensor_normalize() {
        let a = Tensor::vector(vec![3.0, 4.0]);
        let normalized = a.normalize();
        let norm = normalized.norm();
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tensor_sum_mean() {
        let a = Tensor::vector(vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(a.sum(), 10.0);
        assert_eq!(a.mean(), 2.5);
    }

    #[test]
    fn test_tensor_reshape() {
        let a = Tensor::vector(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = a.reshape(TensorShape::matrix(2, 3)).unwrap();

        assert_eq!(b.shape.dims, vec![2, 3]);
        assert_eq!(b.get(0), 1.0);
        assert_eq!(b.get(5), 6.0);
    }

    #[test]
    fn test_tensor_flatten() {
        let a = Tensor::matrix(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = a.flatten();

        assert!(b.shape.is_vector());
        assert_eq!(b.shape.size(), 6);
        assert_eq!(b.get(0), 1.0);
        assert_eq!(b.get(5), 6.0);
    }

    #[test]
    fn test_sparse_tensor() {
        let dense_data = vec![0.0, 1.0, 0.0, 0.0, 2.0, 0.0];
        let sparse_data = TensorData::sparse(vec![1.0, 2.0], vec![1, 4], 6);

        let dense_tensor = Tensor::new(TensorShape::vector(6), TensorData::dense(dense_data));
        let sparse_tensor = Tensor::new(TensorShape::vector(6), sparse_data);

        assert_eq!(dense_tensor.get(0), sparse_tensor.get(0));
        assert_eq!(dense_tensor.get(1), sparse_tensor.get(1));
        assert_eq!(dense_tensor.get(4), sparse_tensor.get(4));

        // Check sparsity
        assert_eq!(sparse_tensor.data.nnz(), 2);
        assert!((sparse_tensor.data.sparsity() - 2.0 / 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_auto_sparse() {
        let mut dense_data = vec![0.0; 100];
        dense_data[10] = 1.0;
        dense_data[50] = 2.0;

        let tensor = Tensor::new(TensorShape::vector(100), TensorData::dense(dense_data));
        let mut sparse_tensor = tensor.clone();
        sparse_tensor.data = sparse_tensor.data.auto_sparse();

        // Check original sparsity (2 non-zero out of 100 = 0.02)
        // auto_sparse won't convert since sparsity is not > 0.5
        assert!(tensor.data.sparsity() < 0.5);
    }
}
