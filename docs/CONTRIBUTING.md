# Contributing to Anya Core

We welcome contributions to the Anya Core project! This document provides guidelines for contributing to the project, including how to maintain code consistency across the project.

## Getting Started

1. Fork the repository on GitHub: <https://github.com/botshelomokoka/anya-core>
2. Clone your fork locally: `git clone https://github.com/your-username/anya-core.git`
3. Create a new branch for your feature or bug fix: `git checkout -b your-branch-name`

## Making Changes

1. Make your changes in your branch.
2. Follow the coding style and conventions used in the project.
3. Write or update tests as necessary.
4. Ensure all tests pass: `cargo test`
5. Run the formatter: `cargo fmt`
6. Run the linter: `cargo clippy`

## Project Structure

Familiarize yourself with the project structure:

anya-core/
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── README.md
├── src/
│   ├── main_system.rs
│   ├── network_discovery.rs
│   ├── user_management.rs
│   ├── stx_support.rs
│   ├── bitcoin_support.rs
│   ├── lightning_support.rs
│   ├── dlc_support.rs
│   ├── kademlia.rs
│   ├── setup_project.rs
│   ├── setup_check.rs
│   └── ml_logic/
│       ├── mod.rs
│       ├── federated_learning.rs
│       └── system_evaluation.rs
├── tests/
│   ├── integration_tests.rs
│   └── unit_tests/
│       ├── user_management_tests.rs
│       ├── blockchain_integration_tests.rs
│       └── ml_logic_tests.rs
├── docs/
│   ├── API.md
│   └── CONTRIBUTING.md
└── scripts/
    ├── setup.sh
    └── run_tests.sh

## Submitting Changes

1. Push your changes to your fork on GitHub.
2. Create a pull request from your branch to the main Anya Core repository.
3. Describe your changes in the pull request description.
4. Reference any related issues in the pull request description.
5. Ensure your code aligns with the project structure and follows the established patterns.

## Code Review

Your pull request will be reviewed by the maintainers. They may suggest changes or improvements. Please be patient and responsive during this process.

## Reporting Issues

If you find a bug or have a suggestion for improvement:

1. Check if the issue already exists in the GitHub issue tracker.
2. If not, create a new issue with a clear title and description.
3. Include steps to reproduce the issue if it's a bug.
4. If possible, provide a minimal code example that demonstrates the issue.

## Key Areas for Contribution

- Enhancing the ML-driven components in `src/ml_logic/`
- Improving Bitcoin, Stacks, and Lightning Network integrations
- Expanding the capabilities of the Discreet Log Contracts (DLCs) support
- Optimizing the Kademlia DHT implementation for better network discovery
- Enhancing privacy features and security measures
- Improving documentation and test coverage

## Code of Conduct

Please note that this project is released with a Contributor Code of Conduct. By participating in this project you agree to abide by its terms.

Thank you for contributing to Anya Core and helping build a revolutionary Bitcoin intelligence platform!