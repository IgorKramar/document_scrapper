# Makefile

# Путь к папке с Node.js проектом
NODE_DIR = ./node_scripts

# Основная цель, которая запускает все шаги
all: node_install node_build cargo_build

# Цель для установки зависимостей npm
node_install:
	cd $(NODE_DIR) && npm install

# Цель для сборки проекта с использованием npm
node_build: node_install
	cd $(NODE_DIR) && npm run build

# Цель для сборки проекта на Rust
cargo_build:
	cargo build --release
