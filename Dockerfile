FROM scorpil/rust:1.13

RUN apt-get update && \
    apt-get install \
       libssl-dev \
       -qqy \
       --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

