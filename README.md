### Prerequisites

Ensure you have the following installed:

- **Rust and Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Git**: [Install Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

### Installation

Clone the repository and navigate into the project directory:

```bash
git clone https://github.com/username/repository.git
cd repository
```

### Build and run the project:

```
cargo build
```

Run the project using short arguments:
```
cargo run -- -a 10 -b 5 -o add
```

Run the project using long arguments:
```
cargo run -- --num1 10 --num2 5 --operation add
```

Sample Output:
```
Result: 15
```
