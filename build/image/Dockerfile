# syntax=docker/dockerfile:1.7.0
FROM ubuntu:25.04
ARG K0SCTL_VERSION="0.22.0"
ENV DEBIAN_FRONTEND=noninteractive
COPY config/journal-to-tty.service /lib/systemd/system/journal-to-tty.service
RUN <<eof
ARCH="$(dpkg --print-architecture)"
apt-get update -yq
apt-get install -y \
  bash \
  ca-certificates \
  conntrack \
  cron \
  curl \
  dbus \
  ethtool \
  file \
  fuse-overlayfs \
  gettext \
  git \
  git \
  gnupg2 \
  iproute2 \
  iptables \
  jq \
  kmod \
  libseccomp2 \
  mount \
  nfs-common \
  nftables \
  open-iscsi \
  openssl \
  pigz \
  procps \
  software-properties-common \
  ssh \
  sudo \
  systemd \
  udev \
  util-linux \
  vim \
  zsh
systemctl enable journal-to-tty.service
useradd -m devcontainer
echo "devcontainer ALL=(ALL) NOPASSWD: ALL" >/etc/sudoers.d/devcontainer
chmod 0440 /etc/sudoers.d/devcontainer
curl -fsSLo /tmp/k0sctl "https://github.com/k0sproject/k0sctl/releases/download/v${K0SCTL_VERSION}/k0sctl-linux-${ARCH}"
install -m 755 /tmp/k0sctl /usr/local/bin/k0sctl
apt-get clean
rm -rf /var/lib/apt/lists/*
eof
CMD [ "/sbin/init" ]
USER devcontainer
HEALTHCHECK NONE
