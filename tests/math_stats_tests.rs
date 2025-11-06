use aether::builtins::math;
use aether::value::Value;

/// 辅助函数：将Value转换为f64
fn to_f64(value: &Value) -> f64 {
    match value {
        Value::Number(n) => *n,
        _ => panic!("Expected number"),
    }
}

/// 辅助函数：将Value转换为Vec<f64>
fn to_vec_f64(value: &Value) -> Vec<f64> {
    match value {
        Value::Array(arr) => arr.iter().map(to_f64).collect(),
        _ => panic!("Expected array"),
    }
}

/// 辅助函数：将Value转换为Vec<Vec<f64>>
fn to_matrix(value: &Value) -> Vec<Vec<f64>> {
    match value {
        Value::Array(arr) => arr.iter().map(to_vec_f64).collect(),
        _ => panic!("Expected matrix"),
    }
}

/// 测试线性回归
#[test]
fn test_linear_regression() {
    // 测试简单的线性关系: y = 2x + 1
    let x = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
    ]);

    let y = Value::Array(vec![
        Value::Number(3.0),
        Value::Number(5.0),
        Value::Number(7.0),
        Value::Number(9.0),
        Value::Number(11.0),
    ]);

    match math::linear_regression(&[x, y]) {
        Ok(Value::Array(arr)) => {
            assert_eq!(arr.len(), 3);
            let slope = to_f64(&arr[0]);
            let intercept = to_f64(&arr[1]);
            let r_squared = to_f64(&arr[2]);

            // 验证斜率接近2
            assert!(
                (slope - 2.0).abs() < 1e-10,
                "Slope should be ~2.0, got {}",
                slope
            );
            // 验证截距接近1
            assert!(
                (intercept - 1.0).abs() < 1e-10,
                "Intercept should be ~1.0, got {}",
                intercept
            );
            // 验证R²接近1（完美拟合）
            assert!(
                (r_squared - 1.0).abs() < 1e-10,
                "R² should be ~1.0, got {}",
                r_squared
            );
        }
        Ok(v) => panic!("Expected array, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }

    // 测试不完美拟合
    let x2 = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
    ]);

    let y2 = Value::Array(vec![
        Value::Number(2.0),
        Value::Number(4.0),
        Value::Number(5.0),
        Value::Number(4.0),
        Value::Number(5.0),
    ]);

    match math::linear_regression(&[x2, y2]) {
        Ok(Value::Array(arr)) => {
            assert_eq!(arr.len(), 3);
            let r_squared = to_f64(&arr[2]);
            // R²应该小于1（非完美拟合）
            assert!(
                r_squared < 1.0 && r_squared > 0.0,
                "R² should be between 0 and 1, got {}",
                r_squared
            );
        }
        Ok(v) => panic!("Expected array, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }
}

/// 测试正态分布PDF
#[test]
fn test_normal_pdf() {
    // 标准正态分布在x=0处的PDF应该是1/sqrt(2π) ≈ 0.3989
    match math::normal_pdf(&[Value::Number(0.0)]) {
        Ok(Value::Number(p)) => {
            let expected = 1.0 / (2.0 * std::f64::consts::PI).sqrt();
            assert!(
                (p - expected).abs() < 1e-6,
                "Expected {}, got {}",
                expected,
                p
            );
        }
        Ok(v) => panic!("Expected number, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }

    // 自定义均值和标准差: x=10, μ=10, σ=2
    // 在均值处，PDF = 1/(σ√(2π))
    match math::normal_pdf(&[Value::Number(10.0), Value::Number(10.0), Value::Number(2.0)]) {
        Ok(Value::Number(p)) => {
            let expected = 1.0 / (2.0 * (2.0 * std::f64::consts::PI).sqrt());
            assert!(
                (p - expected).abs() < 1e-6,
                "Expected {}, got {}",
                expected,
                p
            );
        }
        Ok(v) => panic!("Expected number, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }
}

/// 测试正态分布CDF
#[test]
fn test_normal_cdf() {
    // 标准正态分布在x=0处的CDF应该是0.5
    match math::normal_cdf(&[Value::Number(0.0)]) {
        Ok(Value::Number(p)) => {
            assert!((p - 0.5).abs() < 1e-6, "Expected 0.5, got {}", p);
        }
        Ok(v) => panic!("Expected number, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }

    // 标准正态分布在x=1.96处的CDF应该约为0.975（95%置信区间）
    match math::normal_cdf(&[Value::Number(1.96)]) {
        Ok(Value::Number(p)) => {
            assert!((p - 0.975).abs() < 0.001, "Expected ~0.975, got {}", p);
        }
        Ok(v) => panic!("Expected number, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }

    // 自定义均值和标准差
    match math::normal_cdf(&[Value::Number(15.0), Value::Number(10.0), Value::Number(5.0)]) {
        Ok(Value::Number(p)) => {
            // x=15, μ=10, σ=5 相当于标准正态分布的 z=1
            // CDF(1) ≈ 0.8413
            assert!((p - 0.8413).abs() < 0.001, "Expected ~0.8413, got {}", p);
        }
        Ok(v) => panic!("Expected number, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }
}

/// 测试泊松分布PMF
#[test]
fn test_poisson_pmf() {
    // λ=3, k=2 的泊松概率
    // P(X=2) = (3^2 * e^(-3)) / 2! = 9 * e^(-3) / 2 ≈ 0.224
    match math::poisson_pmf(&[Value::Number(2.0), Value::Number(3.0)]) {
        Ok(Value::Number(p)) => {
            let expected = 9.0 * (-3.0_f64).exp() / 2.0;
            assert!(
                (p - expected).abs() < 1e-6,
                "Expected {}, got {}",
                expected,
                p
            );
        }
        Ok(v) => panic!("Expected number, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }

    // λ=5, k=5 应该是最大概率点
    match math::poisson_pmf(&[Value::Number(5.0), Value::Number(5.0)]) {
        Ok(Value::Number(p)) => {
            // 这应该是一个合理的概率值（0-1之间）
            assert!(
                p > 0.0 && p < 1.0,
                "Probability should be between 0 and 1, got {}",
                p
            );
            // λ=k时概率最大，约为 1/sqrt(2πλ) ≈ 0.178
            assert!((p - 0.178).abs() < 0.01, "Expected ~0.178, got {}", p);
        }
        Ok(v) => panic!("Expected number, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }
}

/// 测试矩阵求逆
#[test]
fn test_matrix_inverse() {
    // 测试2x2矩阵求逆
    // A = [[4, 7], [2, 6]]
    // A^(-1) = [[0.6, -0.7], [-0.2, 0.4]]
    let matrix = Value::Array(vec![
        Value::Array(vec![Value::Number(4.0), Value::Number(7.0)]),
        Value::Array(vec![Value::Number(2.0), Value::Number(6.0)]),
    ]);

    match math::matrix_inverse(&[matrix]) {
        Ok(Value::Array(arr)) => {
            let result = to_matrix(&Value::Array(arr));
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].len(), 2);

            // 验证逆矩阵的元素
            assert!((result[0][0] - 0.6).abs() < 1e-10);
            assert!((result[0][1] - (-0.7)).abs() < 1e-10);
            assert!((result[1][0] - (-0.2)).abs() < 1e-10);
            assert!((result[1][1] - 0.4).abs() < 1e-10);
        }
        Ok(v) => panic!("Expected matrix, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }

    // 测试3x3矩阵求逆 - 验证 A * A^(-1) = I
    let matrix3x3 = Value::Array(vec![
        Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]),
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(1.0),
            Value::Number(4.0),
        ]),
        Value::Array(vec![
            Value::Number(5.0),
            Value::Number(6.0),
            Value::Number(0.0),
        ]),
    ]);

    match math::matrix_inverse(&[matrix3x3.clone()]) {
        Ok(inv_matrix) => {
            // 计算 A * A^(-1)
            match math::matmul(&[matrix3x3, inv_matrix]) {
                Ok(Value::Array(arr)) => {
                    let result = to_matrix(&Value::Array(arr));

                    // 验证对角线元素接近1
                    assert!((result[0][0] - 1.0).abs() < 1e-9);
                    assert!((result[1][1] - 1.0).abs() < 1e-9);
                    assert!((result[2][2] - 1.0).abs() < 1e-9);

                    // 验证非对角线元素接近0
                    assert!(result[0][1].abs() < 1e-9);
                    assert!(result[0][2].abs() < 1e-9);
                    assert!(result[1][0].abs() < 1e-9);
                    assert!(result[1][2].abs() < 1e-9);
                    assert!(result[2][0].abs() < 1e-9);
                    assert!(result[2][1].abs() < 1e-9);
                }
                Ok(v) => panic!("Expected matrix, got: {:?}", v),
                Err(e) => panic!("Error in matmul: {}", e),
            }
        }
        Ok(v) => panic!("Expected matrix, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }
}

/// 测试大矩阵行列式
#[test]
fn test_large_determinant() {
    // 测试4x4矩阵行列式
    let matrix4x4 = Value::Array(vec![
        Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
        ]),
        Value::Array(vec![
            Value::Number(2.0),
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]),
        Value::Array(vec![
            Value::Number(3.0),
            Value::Number(2.0),
            Value::Number(1.0),
            Value::Number(2.0),
        ]),
        Value::Array(vec![
            Value::Number(4.0),
            Value::Number(3.0),
            Value::Number(2.0),
            Value::Number(1.0),
        ]),
    ]);

    match math::determinant(&[matrix4x4]) {
        Ok(Value::Number(det)) => {
            // 这个矩阵的行列式应该是-20
            assert!((det - (-20.0)).abs() < 1e-10, "Expected -20, got {}", det);
        }
        Ok(v) => panic!("Expected number, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }

    // 测试5x5单位矩阵的行列式应该是1
    let identity5x5 = Value::Array(vec![
        Value::Array(vec![
            Value::Number(1.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
        ]),
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(1.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
        ]),
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(1.0),
            Value::Number(0.0),
            Value::Number(0.0),
        ]),
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(1.0),
            Value::Number(0.0),
        ]),
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(1.0),
        ]),
    ]);

    match math::determinant(&[identity5x5]) {
        Ok(Value::Number(det)) => {
            assert!((det - 1.0).abs() < 1e-10, "Expected 1, got {}", det);
        }
        Ok(v) => panic!("Expected number, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }

    // 测试4x4零矩阵的行列式应该是0
    let zero4x4 = Value::Array(vec![
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
        ]),
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
        ]),
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
        ]),
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.0),
        ]),
    ]);

    match math::determinant(&[zero4x4]) {
        Ok(Value::Number(det)) => {
            assert!(det.abs() < 1e-10, "Expected 0, got {}", det);
        }
        Ok(v) => panic!("Expected number, got: {:?}", v),
        Err(e) => panic!("Error: {}", e),
    }
}

/// 测试奇异矩阵求逆应该失败
#[test]
fn test_singular_matrix_inverse() {
    // 奇异矩阵（行列式为0）不可逆
    let singular = Value::Array(vec![
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::Array(vec![Value::Number(2.0), Value::Number(4.0)]),
    ]);

    match math::matrix_inverse(&[singular]) {
        Err(e) => {
            let err_msg = format!("{}", e);
            assert!(
                err_msg.contains("奇异矩阵") || err_msg.contains("singular"),
                "Expected singular matrix error, got: {}",
                err_msg
            );
        }
        Ok(v) => panic!("Expected error for singular matrix, got: {:?}", v),
    }
}

/// 测试线性回归错误处理
#[test]
fn test_linear_regression_errors() {
    // 数组长度不匹配
    let x = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);

    let y = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);

    match math::linear_regression(&[x, y]) {
        Err(e) => {
            let err_msg = format!("{}", e);
            assert!(
                err_msg.contains("长度") || err_msg.contains("length"),
                "Expected length error, got: {}",
                err_msg
            );
        }
        Ok(v) => panic!("Expected error for mismatched lengths, got: {:?}", v),
    }

    // 点数不足
    let x2 = Value::Array(vec![Value::Number(1.0)]);
    let y2 = Value::Array(vec![Value::Number(2.0)]);

    match math::linear_regression(&[x2, y2]) {
        Err(e) => {
            let err_msg = format!("{}", e);
            // 接受两种可能的错误消息
            assert!(
                err_msg.contains("至少") || err_msg.contains("需要") || err_msg.contains("require"),
                "Expected insufficient points error, got: {}",
                err_msg
            );
        }
        Ok(v) => panic!("Expected error for insufficient points, got: {:?}", v),
    }
}
