FROM node:15.0.1-alpine3.10

WORKDIR /web

RUN npm install -g @vue/cli

COPY . .

RUN  cd politeia/templates && npm install && npm run build

RUN cd ../

FROM rust:latest

WORKDIR /web

COPY --from=0 /web .

RUN cd ./politeia

RUN cargo build --release  --package politeia --bin politeia --target-dir .

ENV PORT=8080

EXPOSE 8080

ENTRYPOINT ./release/politeia 