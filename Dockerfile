FROM node:latest

WORKDIR /web

RUN npm install -g @vue/cli

COPY . .

RUN  cd politeia/templates && npm install && npm run build

FROM rust:latest

WORKDIR /web

ENV PORT=8080

ENV VUE_APP_BACKEND_SERVER=http://127.0.0.1:8080

EXPOSE 8080

COPY --from=0 /web .

RUN cargo build --release  --package politeia --bin politeia --target-dir .

ENTRYPOINT ./release/politeia