metaverse_project/
├── .git/                     # Git repository data
├── .gitignore                # Specifies intentionally untracked files that Git should ignore
├── README.md                 # Project overview, setup instructions, etc.
│
├── backend/                  # Rust Cargo Workspace for all backend services
│   ├── Cargo.toml            # Workspace manifest (defines members like services and shared libs)
│   │
│   ├── services/             # Individual microservices
│   │   ├── auth_service/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs         # Service entry point
│   │   │       ├── server.rs       # Tonic gRPC service implementation
│   │   │       └── db.rs           # Database interaction logic (optional, or in shared)
│   │   │
│   │   ├── player_service/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       └── server.rs
│   │   │
│   │   ├── world_service/      # Manages game state, interactions, proximity
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       ├── server.rs
│   │   │       └── spatial.rs    # Spatial indexing logic (e.g., quadtree)
│   │   │
│   │   ├── chat_service/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       └── server.rs
│   │   │
│   │   └── webrtc_signaling_service/ # Handles WebRTC offer/answer/candidate exchange
│   │       ├── Cargo.toml            # Could also be part of world_service
│   │       └── src/
│   │           ├── main.rs
│   │           └── server.rs
│   │
│   ├── shared/                 # Shared Rust crates used by backend services
│   │   ├── proto_definitions/    # Crate for .proto files and generated Rust code
│   │   │   ├── Cargo.toml
│   │   │   ├── build.rs          # Script to run tonic_build for .proto compilation
│   │   │   └── proto/            # Directory containing all your .proto files
│   │   │       ├── auth.proto
│   │   │       ├── player.proto
│   │   │       ├── world.proto
│   │   │       ├── chat.proto
│   │   │       └── webrtc_signal.proto
│   │   │
│   │   ├── common_utils/       # Common utilities, error types, configs
│   │   │   ├── Cargo.toml
│   │   │   └── src/lib.rs
│   │   │
│   │   └── db_access/          # Shared database access logic, models, connection pooling
│   │       ├── Cargo.toml
│   │       ├── src/
│   │       │   ├── lib.rs
│   │       │   ├── models.rs     # Database entity structs
│   │       │   └── schema.rs     # If using Diesel
│   │       └── migrations/       # SQL migration files (for SQLx or Diesel)
│   │           ├── YYYYMMDDHHMMSS_create_users_table.sql
│   │           ├── YYYYMMDDHHMMSS_create_avatars_table.sql
│   │           └── ...
│   │
│   └── .env.example            # Example environment variables for backend services
│
├── frontend/
│   ├── react_app/              # Main React frontend application
│   │   ├── public/
│   │   │   ├── index.html
│   │   │   ├── favicon.ico
│   │   │   └── assets/           # Static assets like images, audio files for the world
│   │   │       ├── sprites/
│   │   │       ├── tilesets/
│   │   │       └── audio/
│   │   ├── src/
│   │   │   ├── App.tsx           # Main React App component
│   │   │   ├── main.tsx          # Entry point for React
│   │   │   ├── components/       # UI components (e.g., WorldView, Chat, Inventory)
│   │   │   │   ├── world/
│   │   │   │   │   └── WorldView.tsx
│   │   │   │   ├── ui/
│   │   │   │   │   ├── ChatWindow.tsx
│   │   │   │   │   └── PlayerMenu.tsx
│   │   │   │   └── ...
│   │   │   ├── hooks/            # Custom React hooks
│   │   │   ├── contexts/         # React Context API providers
│   │   │   ├── services/         # Client-side service integrations
│   │   │   │   ├── GrpcClient.ts # Setup for gRPC-web client
│   │   │   │   ├── AuthService.ts
│   │   │   │   ├── WorldService.ts
│   │   │   │   └── WebRTCManager.ts # Logic for handling WebRTC connections
│   │   │   ├── store/            # State management (e.g., Redux, Zustand slices/stores)
│   │   │   ├── styles/           # Global styles, CSS modules
│   │   │   ├── types/            # TypeScript type definitions
│   │   │   └── proto_gen_ts/     # Output directory for TypeScript code generated from .proto
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── vite.config.ts      # Or craco.config.js / webpack.config.js
│   │   └── .env.example          # Example environment variables for frontend
│   │
│   └── wasm_module/            # Optional Rust/WASM module for frontend logic/rendering
│       ├── Cargo.toml
│       ├── src/lib.rs          # Rust code to be compiled to WASM
│       ├── pkg/                # Output directory for wasm-pack build (gitignore this)
│       └── .gitignore
│
├── docker/                     # Docker related files
│   ├── backend/                # Dockerfiles for each backend Rust service
│   │   ├── auth_service.Dockerfile
│   │   ├── player_service.Dockerfile
│   │   ├── world_service.Dockerfile
│   │   ├── chat_service.Dockerfile
│   │   └── webrtc_signaling_service.Dockerfile
│   │
│   ├── frontend/
│   │   └── react_app.Dockerfile  # Dockerfile to build and serve the React app (e.g., using Nginx)
│   │
│   ├── api_gateway/            # If using a dedicated API Gateway like Envoy or Nginx
│   │   ├── Dockerfile
│   │   └── envoy.yaml          # Example Envoy configuration (if using Envoy)
│   │
│   └── coturn/                 # Configuration for Coturn (STUN/TURN server)
│       └── turnserver.conf     # Coturn configuration file (to be mounted into the container)
│
├── docker-compose.yml          # Defines and orchestrates all services (backend, frontend, DB, cache, etc.)
│
└── scripts/                    # Utility scripts
    ├── build_all.sh            # Script to build all services and frontend
    ├── start_dev.sh            # Script to run services in development mode (e.g., using docker-compose up)
    ├── stop_dev.sh
    ├── gen_proto_rust.sh       # Script to run `tonic_build` for Rust
    └── gen_proto_ts.sh         # Script to generate TypeScript gRPC clients
