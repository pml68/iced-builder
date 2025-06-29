# Maintainer: pml68 <contact@pml68.dev>

pkgname=iced-builder
pkgver=0.1.0.r187.g2bbe61b
pkgrel=1
pkgdesc='UI builder for iced, built with iced.'
arch=(x86_64)
url='https://github.com/pml68/iced-builder'
license=('GPL-3.0-or-later')
depends=(
  gcc-libs
  glibc
  openssl
)
optdepends=('rustfmt: better code formatting')
makedepends=(
  git
  cargo
  clang
  mold
)
options=('!lto' '!strip' '!debug')
source=("$pkgname::git+${url}.git")
sha256sums=('SKIP')

prepare() {
  cd "${pkgname}"

  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

pkgver() {
  cd "${pkgname}"
  printf "%s.r%s.g%s" \
    "$(cargo pkgid | cut -d@ -f2)" \
    "$(git rev-list --count HEAD)" \
    "$(git rev-parse --short HEAD)"
}

build() {
  cd "${pkgname}"

  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  export RUSTFLAGS="${RUSTFLAGS} --remap-path-prefix $srcdir=src"
  cargo build --frozen --release
}

check() {
  cd "${pkgname}"

  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo test --frozen --release
}

package() {
  cd "${pkgname}"

  install -Dm755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"

  install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
  install -Dm644 "assets/linux/dev.pml68.${pkgname}.desktop" -t "${pkgdir}/usr/share/applications"
}
