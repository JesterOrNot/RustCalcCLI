FROM golang AS clone

RUN git clone "https://github.com/JesterOrNot/RustCalcCLI.git" /app

FROM rust:slim@sha256:96d44debbfd641d7873d11242e287bfba85fd21ae03e83c6eb2a0b27bf55d4fc AS build
COPY --from=clone /app/RustCalcCLI /app

WORKDIR /app
RUN cargo build --release

FROM scratch
COPY --from=build /app/RustCalcCLI/target/release/rustcalc /bin
CMD [ "/bin/sh" ]
