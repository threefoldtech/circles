# Circle Collaboration System

A modern, circle-based collaboration platform built with Rust and egui.

## Overview

Circle is a desktop application that provides a modular collaboration system organized around "Circles" - dedicated spaces for different contexts of work and communication. Each Circle contains a consistent set of collaboration features, allowing users to seamlessly switch between different work contexts while maintaining a familiar interface.

## Features

- **Circle Management**: Create and manage different types of circles (Personal, Team, Private)
- **Mail**: Send and receive messages within circles
- **Calendar**: Schedule and manage events
- **Chat**: Real-time messaging
- **Documents**: Create and collaborate on documents
- **AI Tools**: AI-powered assistance
- **Video Conferencing**: Real-time video communication

## Architecture

The application follows a modular architecture:

- **Core Components**: Application layer, Circle Manager, and Circle Structure
- **UI/UX**: Consistent layout across features with a Circle Selector and Feature Navigation
- **Data Model**: Structured representation of Circles, Members, and Features
- **Services**: Backend services for Circle management, authentication, and feature-specific operations

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)

### Building from Source

1. Clone the repository:
   ```
   git clone https://github.com/threefoldtech/circles.git
   cd circle
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run --release
   ```

## Usage

Upon launching the application, you'll be presented with a default user and several demo circles. You can:

1. Select a circle from the Circle Selector panel
2. Navigate between features using the feature tabs
3. Create new circles using the Circle Selector
4. Customize circle settings


## Dependencies

- **egui/eframe**: GUI framework for Rust
- **uuid**: For unique identifiers
- **chrono**: Date and time handling
- **serde/serde_json**: Serialization and deserialization
- **log/env_logger**: Logging

## Development Status

This project is currently in early development. Many features are implemented as UI mockups with dummy data.

## Future Plans

- Cloud synchronization
- End-to-end encryption
- Real-time collaboration
- Mobile support via WebAssembly

## License

[MIT License](LICENSE) (placeholder - replace with actual license)
