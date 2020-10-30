FROM rust:latest
FROM node:latest

WORKDIR /web

COPY . .

RUN cd ./politeia

RUN npm install

RUN npm run build

RUN cargo build --release  --package politeia --bin politeia > Executable

ENV PORT=8080

EXPOSE 8080

ENTRYPOINT ./Executable