# Simple Code Parser

This project implements a parser for a context-free language that adheres to the following grammar:

```
program -> stmt (stmt)*
stmt -> print | assign
print -> "print" expr
assign -> "let" VAR "=" expr
expr -> NUM | VAR | "(" expr ")" | expr ("+"|"*") expr
NUM -> 0 | [1-9]+[0-9]*
VAR -> [a-z]+
```

The parser generates an Abstract Syntax Tree (AST) from the input code and provides a graphical visualization of the tree structure.

## Features
- Parses a custom context-free grammar.
- Outputs an AST to the console.
- Visualizes the AST as a PNG file using Graphviz.
- Includes test cases for CI to validate the parser.

---

## Installation

### Prerequisites
1. Ensure you have Python installed (version 3.8 or higher is recommended).
2. Install [Graphviz](https://graphviz.org/). Follow the instructions for your operating system. To verify the installation, run:
   ```sh
   dot -v
   ```
   This will display the version of Graphviz if it is installed correctly.

### Setting Up the Environment
1. Clone the repository:
   ```sh
   git clone https://github.com/vlapugb/Simple-code-parser.git
   cd Simple-code-parser
   ```
2. Create a virtual environment:
   ```sh
   python -m venv venv
   ```
3. Activate the virtual environment:
    - On Windows:
      ```sh
      venv\Scripts\activate
      ```
    - On macOS/Linux:
      ```sh
      source venv/bin/activate
      ```
4. Install the required dependencies:
   ```sh
   pip install -r requirements.txt
   ```

---

## Usage

### Running the Parser
To parse a file, use the following command:
```sh
cargo run -- --filename "path_to_file_from_the_project_root"
```
- Replace `path_to_file_from_project_root` with the relative path to your text file containing the code.

### Output
1. The AST will be printed to the console.
2. A PNG visualization of the AST will be generated and saved as `AST_graphviz.png` in the root directory of the project.

---

## Testing

Tests are included in the `tests` directory. To run the tests, use:
```sh
cargo test
```
This will execute all test cases and verify the correctness of the parser.

---

## Example

### Input File (`example.txt`):
```
let z = (y + 6) * 2
```

### Command:
```sh
python main.py --filename "example.txt"
```

### Console Output:
```
digraph AST {
node0 [label="Root"];
node1 [label="Assign(z)"];
node2 [label="Mul"];
node3 [label="Add"];
node4 [label="Var(y)"];
node5 [label="Num(6)"];
node3 -> node4;
node3 -> node5;
node6 [label="Num(2)"];
node2 -> node3;
node2 -> node6;
node1 -> node2;
node0 -> node1;
}


```

### Generated File:
`AST_graphviz.png` - a visual representation of the AST.

---

## Notes
- Ensure Graphviz is installed and correctly configured to generate the visualization.
- The parser supports basic error handling, but invalid inputs may result in a crash. Please use valid inputs based on the specified grammar.

---

## License
This project is open-source and available under the MIT License.

---

## Acknowledgments
Thanks for using this parser. If you encounter any issues or have suggestions for improvement, feel free to open an issue or submit a pull request!

