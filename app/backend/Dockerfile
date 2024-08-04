# syntax=docker.io/docker/dockerfile:1.7-labs

FROM rust:bookworm as builder
WORKDIR /usr/src/backend
COPY --exclude=target --exclude=runtime . .
RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y nodejs npm libasound2 libatk1.0-0 libc6 libcairo2 libcups2 libdbus-1-3 libexpat1 libfontconfig1 libgcc1 libgconf-2-4 libgdk-pixbuf2.0-0 libglib2.0-0 libgtk-3-0 libnspr4 libpango-1.0-0 libpangocairo-1.0-0 libstdc++6 libx11-6 libx11-xcb1 libxcb1 libxcomposite1 libxcursor1 libxdamage1 libxext6 libxfixes3 libxi6 libxrandr2 libxrender1 libxss1 libxtst6 ca-certificates fonts-liberation libnss3 lsb-release xdg-utils wget ca-certificates

RUN useradd -m localuser 
WORKDIR /home/localuser 

COPY --from=builder /usr/local/cargo/bin/backend .
COPY --exclude=sidexporter/node_modules sidexporter sidexporter

RUN chown -R localuser:localuser /home/localuser 

WORKDIR /home/localuser/sidexporter
USER localuser

RUN npm install
RUN npx puppeteer browsers install chrome

# RUN chown -R localuser:localuser /home/localuser 

WORKDIR /home/localuser

CMD ["./backend"]
