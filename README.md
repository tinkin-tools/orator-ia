# Orator IA

A Tauri-based desktop application for audio processing and management with AI integration capabilities.

## 🏗️ Architecture Overview

This project follows a **Layered Architecture** pattern with **Domain-Driven Design** principles, ensuring clean separation of concerns, maintainability, and testability.

### Architecture Layers

```
┌─────────────────────────────────────────────┐
│                Frontend                     │
│            (React + TypeScript)             │
└─────────────────────────────────────────────┘
                       │
┌─────────────────────────────────────────────┐
│              Tauri Bridge                   │
└─────────────────────────────────────────────┘
                       │
┌─────────────────────────────────────────────┐
│          🎯 Presentation Layer               │
│       (Tauri Commands & Handlers)           │
└─────────────────────────────────────────────┘
                       │
┌─────────────────────────────────────────────┐
│          📋 Application Layer                │
│          (Use Cases & Services)              │
└─────────────────────────────────────────────┘
                       │
┌─────────────────────────────────────────────┐
│           🏛️ Domain Layer                    │
│     (Business Logic & Entities)             │
└─────────────────────────────────────────────┘
                       │
┌─────────────────────────────────────────────┐
│         🔧 Infrastructure Layer              │
│    (External APIs, File System, etc.)       │
└─────────────────────────────────────────────┘
```

## 📁 Project Structure

### Backend Architecture (Rust/Tauri)

```
src-tauri/src/
├── lib.rs                    # App composition and dependency injection
├── main.rs                   # Entry point
│
├── presentation/             # 🎯 Presentation Layer
│   ├── commands/             # Tauri command handlers
│   │   ├── audio_commands.rs
│   │   └── settings_commands.rs
│   └── handlers/             # Error handling and response mapping
│       └── error_handler.rs
│
├── application/              # 📋 Application Layer
│   ├── audio/                # Audio use cases
│   │   └── process_audio_use_case.rs
│   └── settings/             # Settings use cases
│       ├── get_settings_use_case.rs
│       ├── update_settings_use_case.rs
│       └── reset_settings_use_case.rs
│
├── domain/                   # 🏛️ Domain Layer
│   ├── audio/                # Audio domain
│   │   ├── entities/         # Domain entities
│   │   │   ├── audio_file.rs
│   │   │   └── processing_config.rs
│   │   ├── repositories/     # Repository contracts (traits)
│   │   │   └── audio_repository.rs
│   │   └── services/         # Domain services
│   │       └── audio_processor_service.rs
│   ├── settings/             # Settings domain
│   │   ├── entities/
│   │   │   ├── config_item.rs
│   │   │   └── settings.rs
│   │   └── repositories/
│   │       └── settings_repository.rs
│   └── shared/               # Shared domain concepts
│       ├── errors.rs
│       └── value_objects.rs
│
├── infrastructure/           # 🔧 Infrastructure Layer
│   ├── audio/                # Audio processing implementations
│   │   └── symphonia_processor.rs
│   ├── settings/             # Settings persistence
│   │   └── file_settings_repository.rs
│   └── filesystem/           # File system operations
│       └── file_operations.rs
│
└── shared/                   # 🛠️ Shared Utilities
    ├── config/               # Application configuration
    └── utils/                # Common utilities
```

## 🔧 Architecture Principles

### 1. **Dependency Inversion**

- Higher-level modules don't depend on lower-level modules
- Both depend on abstractions (traits)
- Infrastructure implements domain contracts

### 2. **Single Responsibility**

- Each layer has a clear, single responsibility
- **Presentation**: Handle user interface interactions
- **Application**: Orchestrate business workflows
- **Domain**: Encapsulate business rules and logic
- **Infrastructure**: Manage external concerns

### 3. **Clean Dependencies**

```rust
Domain ← Application ← Presentation
   ↑
Infrastructure
```

### 4. **Domain-Driven Design**

- Rich domain models with business logic
- Value objects for data validation
- Repository pattern for data access
- Domain services for complex business operations

## 🚀 Key Features

### Audio Processing

- **Silence Detection**: Remove silence from audio files
- **Configurable Parameters**: Customizable thresholds and durations
- **Multiple Formats**: Support for WAV, MP3, FLAC, OGG, M4A
- **Quality Preservation**: High-quality audio processing

### Settings Management

- **Hierarchical Configuration**: System and user settings
- **Environment Override**: Support for environment variables
- **Validation**: Type-safe configuration with validation
- **Persistence**: JSON-based settings storage

## 🛠️ Technology Stack

### Backend

- **Rust**: Systems programming language
- **Tauri**: Desktop application framework
- **Symphonia**: Audio decoding and processing
- **Hound**: WAV file writing
- **Serde**: Serialization/deserialization

### Frontend

- **React**: UI library
- **TypeScript**: Type-safe JavaScript
- **Vite**: Build tool and development server

## 🏃‍♂️ Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v16 or later)
- [pnpm](https://pnpm.io/) (recommended package manager)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd orator-ia

# Install frontend dependencies
pnpm install

# Install Rust dependencies (handled by Cargo)
cd src-tauri
cargo check

# Development
pnpm tauri dev

# Build for production
pnpm tauri build
```

## 🧪 Testing Strategy

### Unit Tests

```bash
# Test domain logic
cargo test --lib domain

# Test use cases
cargo test --lib application

# Test infrastructure
cargo test --lib infrastructure
```

### Integration Tests

```bash
# Test end-to-end workflows
cargo test --test integration
```

## 🔄 Use Case Examples

### Audio Processing Flow

1. **Presentation**: Receives command with parameters
2. **Application**: Validates input and orchestrates processing
3. **Domain**: Applies business rules and validation
4. **Infrastructure**: Performs actual audio processing

```rust
// Use case execution
let use_case = ProcessAudioUseCase::new(audio_repository);
let result = use_case.execute(
    input_path,
    output_path,
    silence_threshold,
    min_silence_duration,
    min_audio_duration,
);
```

### Settings Management Flow

1. **Repository Pattern**: Abstract settings storage
2. **Value Objects**: Type-safe configuration values
3. **Domain Services**: Business logic for settings

## 📚 API Documentation

### Tauri Commands

#### Audio Processing

```typescript
// Process audio file
await invoke("process_audio_file", {
  inputPath: string,
  outputPath: string,
  silenceThreshold: number,
  minSilenceDuration: number,
  minAudioDuration: number,
})
```

#### Settings Management

```typescript
// Get all settings
const settings = await invoke('get_all_settings');

// Update setting
await invoke('update_setting', {
  section: string,
  subsection?: string,
  key: string,
  value: any
});

// Reset to defaults
await invoke('reset_settings_to_defaults');
```

## 🤝 Contributing

### Code Organization Guidelines

1. **Domain First**: Start with domain models and business rules
2. **Interface Segregation**: Define clear contracts between layers
3. **Dependency Injection**: Use constructor injection for dependencies
4. **Error Handling**: Use domain-specific error types
5. **Testing**: Write tests for each layer independently

### Adding New Features

1. Define domain entities and business rules
2. Create repository contracts (traits)
3. Implement use cases in application layer
4. Add infrastructure implementations
5. Create presentation layer commands
6. Update dependency injection in `lib.rs`

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔍 Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
