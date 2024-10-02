FROM fedora:40

RUN dnf install -y python3 python3-pip rust cargo
RUN pip install twscrape

COPY . /app

WORKDIR /app

RUN --mount=type=cache,target=/root/.cargo --mount=type=cache,target=/app/target/release cargo build --release

RUN --mount=type=cache,target=/app/target/release cp target/release/pipe-down-latinx /usr/local/bin/pipe-down-latinx

RUN rm -rf /app /root/.cargo /root/.rustup /root/.cache /var/cache/dnf && mkdir /app 

WORKDIR /data

CMD ["/usr/local/bin/pipe-down-latinx"]