# Build container with:
#
# docker build . --tag particles
#
# Run container with something like:
#
# XSOCK=/tmp/.X11-unix
# XAUTH=/tmp/.docker.xauth
# touch "${XAUTH}"
# xauth nlist :0 | sed -e 's/^..../ffff/' | xauth -f "${XAUTH}" nmerge -
# docker run -ti -v /etc/passwd:/etc/passwd:ro -v /etc/group:/etc/group:ro -v "${XSOCK}:${XSOCK}" -v "${XAUTH}:${XAUTH}" -v /dev/dri/card0:/dev/dri/card0 -e "XAUTHORITY=${XAUTH}" -e DISPLAY -e HOME -e USER --user "$(id -u):$(id -g)" --rm particles
#
FROM rust:1.66

RUN apt-get update
RUN apt-get -y install libxcursor-dev libxrandr2 libxi6 libx11-xcb1 libgl1

WORKDIR /opt/particles
COPY . .
RUN test -f Cargo.lock && rm Cargo.lock || true
RUN cargo build
CMD ["/opt/particles/target/debug/ch6-particles"]
