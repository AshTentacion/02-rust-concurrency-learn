use anyhow::Result;
use concurrency::Matrix;

fn main() -> Result<()> {
    Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    Ok(())
}
