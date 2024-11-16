FROM rust:1 AS build-env
WORKDIR /app
COPY ./crates ./crates
COPY ./Cargo.toml .
COPY .env .
RUN cargo build --release
RUN chmod +x ./target/release/sync

FROM debian:bookworm-slim
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies and Chrome
RUN apt-get update && apt-get install -y \
    wget \
    gnupg \
    ca-certificates \
    fonts-liberation \
    libappindicator3-1 \
    libasound2 \
    libatk1.0-0 \
    libcups2 \
    libdbus-1-3 \
    libdrm2 \
    libx11-xcb1 \
    libxcomposite1 \
    libxdamage1 \
    libxrandr2 \
    libgbm1 \
    libgtk-3-0 \
    xdg-utils \
    --no-install-recommends && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Add Google's signing key and repository
RUN wget -q -O - https://dl.google.com/linux/linux_signing_key.pub | apt-key add - && \
    echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" > /etc/apt/sources.list.d/google-chrome.list

# Install Google Chrome
RUN apt-get update && apt-get install -y google-chrome-stable && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# copy the binary from the build-env image
RUN useradd -ms /bin/bash app_user
USER app_user
WORKDIR /app
RUN chown -R app:app /app
COPY --from=build-env /app/target/release/sync /app/sync
COPY --from=build-env /app/.env /app/.env

# write a script that runs the binary and waits for 24 hours
RUN echo "#!/bin/bash \
    while true; do \
        ./sync; \
        sleep 86400; \
    done" > /app/run.sh
RUN chmod +x /app/run.sh

CMD ["./sync"]
