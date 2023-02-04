FROM rust:1.67-alpine AS backend
WORKDIR /usr/src/progressly
COPY backend .
RUN apk add libc-dev && cargo build --release

# build frontend
FROM rust:1.67-alpine AS frontend
WORKDIR /usr/src/progressly
COPY frontend .
RUN apk add libc-dev \
    && rustup target add wasm32-unknown-unknown \
    && cargo install trunk \
    && trunk build --release

# build backend
FROM alpine
RUN mkdir -p /progressly
WORKDIR /progressly
COPY --from=backend /usr/src/progressly/target/release/progressly /progressly/progressly
COPY --from=frontend /usr/src/progressly/dist /progressly/static
CMD ["progressly"]
