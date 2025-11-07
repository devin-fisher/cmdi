pub mod util {
    use color_eyre::eyre::eyre;

    pub fn try_insert<T>(vec: &mut Vec<T>, index: usize, value: T) -> color_eyre::Result<()> {
        if index <= vec.len() {
            vec.insert(
                index, value,
            );
            Ok(())
        } else {
            Err(
                eyre!(
                    "Index {} out of bounds (len = {})",
                    index,
                    vec.len()
                ),
            )
        }
    }
}
