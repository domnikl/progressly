FROM rust:1.68-alpine AS backend
WORKDIR /app
COPY backend .
RUN apk add libc-dev libpq-dev && cargo install --path .

# build frontend
FROM rust:1.68-alpine AS frontend
WORKDIR /usr/src/progressly
COPY frontend .
RUN apk add libc-dev openssl-dev \
    && rustup target add wasm32-unknown-unknown \
    && cargo install trunk \
    && trunk build --release

# build backend
FROM alpine
WORKDIR /progressly
COPY --from=backend /usr/local/cargo/bin/progressly /usr/local/bin/progressly
COPY --from=frontend /usr/src/progressly/dist/* /app/static/
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["progressly"]
