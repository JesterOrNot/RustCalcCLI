FROM rust AS build
RUN git clone "https://github.com/JesterOrNot/RustCalcCLI.git" /app
WORKDIR /app
RUN cargo build --release
FROM scratch
COPY --from=build /app/RustCalcCLI/target/release/rustcalc /bin
CMD [ "/bin/sh" ]
