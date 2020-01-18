FROM rust:slim-stretch@sha256:96d44debbfd641d7873d11242e287bfba85fd21ae03e83c6eb2a0b27bf55d4fc AS build
WORKDIR /
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        git=1:2.20.1-2ubuntu1
RUN git clone "https://github.com/JesterOrNot/RustCalcCLI.git" /app
WORKDIR /app
RUN cargo build --release
FROM scratch
COPY --from=build /app/RustCalcCLI/target/release/rustcalc /bin
CMD [ "/bin/sh" ]