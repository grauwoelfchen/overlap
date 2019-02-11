# Gentoo Linux container with Rust (latest)
# https://github.com/grauwoelfchen/portolan/blob/master/rust/latest/Dockerfile
FROM grauwoelfchen/rust:latest as app

RUN mkdir /overlap
COPY . /overlap
WORKDIR /overlap
RUN make build:release


FROM gentoo/portage:latest as portage
FROM gentoo/stage3-amd64-nomultilib:latest

COPY --from=portage /usr/portage /usr/portage

RUN mkdir -p /opt/bin
COPY --from=app /overlap/target/release/overlap /opt/bin/overlap
WORKDIR /srv

ENTRYPOINT ["/opt/bin/overlap"]
CMD [""]
