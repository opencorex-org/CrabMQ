# 🦀 CrabMQ

**CrabMQ** is a high-performance, open-source **MQTT broker** written in **Rust**, designed for **scalability, reliability, and safety**.  
It is a community-driven project maintained by **OpenCorex**.

> Built with Rust 🦀 for modern IoT, messaging, and event-driven systems.

---

## ✨ Features

- ⚡ High-performance async networking (Tokio)
- 🧵 Safe concurrency with Rust
- 📡 MQTT v3.1.1 support (in progress)
- 🌲 Topic routing with wildcard support (`+`, `#`)
- 🔁 Publish / Subscribe messaging
- 🧠 Session management
- 🪵 Structured logging with `tracing`
- 🔐 Optional TLS & authentication (planned)
- 📦 Modular, contributor-friendly architecture

---

## 📦 Project Structure

```text
CrabMQ/
├── config/                 # Configuration files (TOML)
├── src/
│   ├── broker/             # Core broker logic
│   ├── protocol/           # MQTT protocol encoding/decoding
│   ├── server/             # TCP server & connection handling
│   ├── security/           # Auth & TLS (optional)
│   ├── utils/              # Helpers (logging, config, timers)
│   ├── errors.rs           # Common error handling
│   └── main.rs             # Application entry point
├── examples/               # Example MQTT clients
├── tests/                  # Unit & integration tests
├── scripts/                # Build & run scripts
└── README.md
```

---

## 🚀 Getting Started

### 1️⃣ Prerequisites

- Rust (stable)

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify:
```bash
rustc --version
cargo --version
```

---

### 2️⃣ Clone the Repository

```bash
git clone https://github.com/opencorex-org/CrabMQ.git
cd CrabMQ
```

---

### 3️⃣ Configure CrabMQ

Edit the default configuration file:

```bash
config/default.toml
```

Example:
```toml
[broker]
host = "0.0.0.0"
port = 1883
max_connections = 10000

[logging]
level = "info"
```

---

### 4️⃣ Build the Project

```bash
cargo build
```

For production:
```bash
cargo build --release
```

---

### 5️⃣ Run the Broker

Using Cargo:
```bash
cargo run
```

With config file:
```bash
cargo run -- --config config/default.toml
```

Or run the binary directly:
```bash
./target/debug/crabmq --config config/default.toml
```

---

## 🧪 Testing with MQTT Clients

Install Mosquitto clients:

### Ubuntu / Debian
```bash
sudo apt install mosquitto-clients
```

### macOS
```bash
brew install mosquitto
```

Subscribe:
```bash
mosquitto_sub -h localhost -p 1883 -t "test/topic"
```

Publish:
```bash
mosquitto_pub -h localhost -p 1883 -t "test/topic" -m "Hello CrabMQ"
```

---

## 🧪 Run Tests

```bash
cargo test
```

---

## 🧠 Roadmap

- [ ] MQTT QoS 1 & 2
- [ ] Retained messages
- [ ] Persistent sessions
- [ ] TLS support (8883)
- [ ] Authentication & ACL
- [ ] WebSocket MQTT
- [ ] Clustering & replication
- [ ] Metrics & observability

---

## 🤝 Contributing

Contributions are welcome! 🎉  

Please open an issue or submit a pull request.

---

## 📜 License

CrabMQ is licensed under the **Apache License 2.0**
