# Build Executable
FROM rustlang/rust:nightly as builder

ADD rs/abenga_site ./abenga_site
ADD configuration.toml ./configuration.toml
WORKDIR ./abenga_site

RUN cargo build --release

# Create Slim Distribution Image
FROM debian:buster-slim
ARG APP=/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /abenga_site/target/release/abenga_site ${APP}/abenga_site
COPY --from=builder /configuration.toml ${APP}/configuration.toml
COPY --from=builder /abenga_site/static ${APP}/static
COPY --from=builder /abenga_site/templates ${APP}/templates

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./abenga_site"]
