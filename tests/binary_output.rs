use std::process::Command;

#[test]
fn binary_prints_expected_demo_output() {
    let binary = env!("CARGO_BIN_EXE_rust_pipeline_demo");
    let output = Command::new(binary)
        .output()
        .expect("failed to run rust_pipeline_demo binary");

    assert!(output.status.success(), "binary exited with failure");

    let stdout = String::from_utf8(output.stdout).expect("binary output was not valid UTF-8");

    assert!(stdout.contains("=== Rust Pipeline Demo ==="));
    assert!(stdout.contains("Arithmétique :"));
    assert!(stdout.contains("add(10, 5)      = 15"));
    assert!(stdout.contains("subtract(10, 5) = 5"));
    assert!(stdout.contains("multiply(10, 5) = 50"));
    assert!(stdout.contains("divide(10, 5)   = Some(2)"));
    assert!(stdout.contains("divide(10, 0)   = None"));
    assert!(stdout.contains("Fibonacci (0..=10) :"));
    assert!(stdout.contains("[0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]"));
    assert!(stdout.contains("Nombres premiers jusqu'à 30 :"));
    assert!(stdout.contains("[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]"));
}