

# Build Rust app
FROM rust:latest as rust-build

WORKDIR /app/server
COPY server/Cargo.toml .
COPY server .
RUN cargo build --release

# Run the backend
FROM mongo:latest
COPY --from=rust-build /app/server/target/release/habits /app/server/server


EXPOSE 8080

CMD mongod & ./app/server/server


# Build React app
# FROM node:14 as react-build
#
# WORKDIR /app/client
# COPY client/package*.json ./
# RUN yarn install
# COPY client .
# RUN yarn build
# EXPOSE 3000
# COPY --from=react-build /app/client/dist /app/client/build

# CMD ["npm", "run", "preview"]

