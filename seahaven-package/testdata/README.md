seahaven-package-testdata
-------------------------

This crate contains test vectors for the Seahaven package manifest.

The test vectors are stored in the `data` directory. 
The templates for the test vectors are stored in the `templates` directory.

## Code generation

The `build.rs` file contains the code that generates the test vectors from the templates and the data files.

To re-generate the test vectors, run the following command:

```sh
cargo build -p seahaven-package-testdata --features=codegen
```

This will generate the test vector files in the `src` directory.
