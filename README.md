# Euclides

> A Rust-based conceptual calculator designed to explore the fundamentals of the language.

Euclides is a simple calculator that performs arithmetic operations using only addition and subtraction. It supports complex expressions and evaluates them using [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) (postfix notation), along with infix and prefix notations.

## Features

- **Basic Operations**: Addition, Subtraction, Multiplication, and Division.
- **Notation Support**: Infix, Prefix, and Postfix notations.
- **Expression Evaluation**: Computes results for given expressions.

## Usage

To use Euclides, run the program and input an expression in one of the supported notations. Specify the notation by prefixing your input with `prefix:`, `postfix:`, or `infix:`.

### Examples

1. **Infix Notation**:
    ```bash
    $ infix: 3 + 4 * 2 / ( 1 - 5 )
    ```

2. **Prefix Notation**:
    ```bash
    $ prefix: + 3 / * 4 2 - 1 5
    ```

3. **Postfix Notation**:
    ```bash
    $ postfix: 3 4 2 * 1 5 - / + 
    ```

## Roadmap

- [x] Implement basic operations
- [x] Support infix, prefix, and postfix notations
- [ ] Add support for advanced/complex operations
- [ ] Develop a graphical user interface (GUI)

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the [BSD Zero Clause](LICENSE) License.
