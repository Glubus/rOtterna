# rOtterna

A desktop application for browsing and downloading Etterna song packs. Built with Tauri, React, and TypeScript.

## Features

- **Search & Browse** - Search through thousands of Etterna packs
- **Download Packs** - Download packs directly to your computer
- **Sort & Filter** - Sort by name, popularity, difficulty ratings, and more
- **Settings** - Configure HP drain rate, overall difficulty, and song path
- **Modern UI** - Beautiful interface built with DaisyUI and Tailwind CSS

## Planned Features

- **Auto Collection Creation** - Automatically create collections in osu! for downloaded packs
- **Auto Rating on Download** - Automatically rate maps during download between two rating ranges

## Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [pnpm](https://pnpm.io/) (package manager)
- [Rust](https://www.rust-lang.org/) (latest stable version)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Glubus/rOtterna
cd rOtterna
```

2. Install dependencies:
```bash
pnpm install
```

## Development

Run the app in development mode:

```bash
pnpm tauri dev
```

This will start the Vite dev server and launch the Tauri application.

## Building

Build the application for production:

```bash
pnpm tauri build
```

The built application will be in `src-tauri/target/release/bundle/`.

## Project Structure

```
rOtterna/
├── src/                    # React frontend
│   ├── components/         # React components
│   ├── hooks/              # Custom React hooks
│   └── App.tsx             # Main app component
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── packs/          # Pack browsing and downloading
│   │   ├── maps/           # Map processing
│   │   └── settings.rs     # Settings management
│   └── Cargo.toml          # Rust dependencies
└── package.json            # Node.js dependencies
```

## Technologies

- **Frontend**: React 19, TypeScript, Vite
- **UI**: Tailwind CSS, DaisyUI, RSC DaisyUI
- **Backend**: Rust, Tauri v2
- **API**: EtternaOnline API

## License

MIT
