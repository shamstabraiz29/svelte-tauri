# CloudCAD Frontend

This repository contains the frontend application for CloudCAD, built with Tauri and TypeScript.

## 📂 Repository Structure

This repository contains two important submodules:

1. **common**: Located at `./common`, sourced from https://github.com/cloudcad/common-shared.git
2. **ui**: Located at `./frontend/ui`, sourced from https://github.com/cloudcad/svelte-ui-v2.git

⚠️ **Important**: Before making changes to either submodule, ensure you create and switch to a new branch within the submodule. This helps maintain proper version control and prevents unintended changes to the main branch.

## 🛠️ Prerequisites

Before you begin, ensure you have the following installed:

### Rust (version 1.80.0 or greater)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup update
```

Verify installation: `rustc --version`

### Node.js (version 22.x)

Download and install from [nodejs.org](https://nodejs.org/)

### pnpm (version 9.4.0 or later)

```sh
npm install -g pnpm@9.4.0
```

### Tauri CLI

```sh
cargo install tauri-cli --version 2.0.0-rc.11 --force
```

### TypeScript (version 5.5.4 or later)

```sh
pnpm add -g typescript@5.5.4
```

Verify installations:

```sh
node --version
pnpm --version
tsc --version
```

## 🚀 Setup and Running

1. Clone the repository with submodules:

   ```sh
   git clone --recursive https://github.com/cloudcad/cloudcad-frontend.git
   cd cloudcad-frontend
   ```

2. Set up submodules:

   ```sh
   git submodule update --init --recursive
   ```

3. Before making changes to a submodule, create and switch to a new branch:

   ```sh
   cd common  # or cd frontend/ui for the ui submodule
   git checkout -b your-new-feature-branch
   cd ..
   ```

4. Install dependencies:

   ```sh
   cd frontend/ui
   pnpm install
   ```

5. Run the development server:

   ```sh
   cd ../native/app
   cargo tauri dev
   ```

This will start the Tauri app in development mode.

## 🐛 Troubleshooting

If you encounter any issues with dependencies or versions, ensure all tools are correctly installed and up-to-date. Refer to the `package.json` file for specific dependency requirements.

For submodule-related issues, make sure you've initialized and updated the submodules correctly.

## 🤝 Contributing

As this is a private repository for CloudCAD, please follow the company's internal guidelines for contributing to this project. Ensure you:

1. Follow the existing code structure and naming conventions.
2. Write clear, descriptive commit messages.
3. Update relevant documentation when making significant changes.
4. Write and update tests as necessary.
5. When working with submodules, always create a new branch before making changes.

## 📄 License

This project is proprietary and confidential. Unauthorized copying, transferring, or reproduction of the contents of this repository, via any medium, is strictly prohibited.

Copyright (C) CloudCAD - All Rights Reserved

