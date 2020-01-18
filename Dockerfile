FROM rust@sha256:e67e6547c7cfb51044d42816c50e2f3c0ab6427158fbb6d3416cedda59867b99 AS build
RUN git clone "https://github.com/JesterOrNot/RustCalcCLI.git" /app
WORKDIR /app
RUN cargo build --release
FROM scratch
COPY --from=build /app/target/release/rustcalc /bin
CMD [ "/bin/sh" ]
