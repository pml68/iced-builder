# Maintainer: pml68 <contact@pml68.dev>

pkgname=iced-builder
_pkgver=0.1.0
pkgver=0.1.0.gb55bd80
pkgrel=1
pkgdesc='UI builder for iced, built with iced.'
arch=(x86_64)
url='https://github.com/pml68/iced-builder'
license=('GPL-3.0-or-later')
depends=(
  gcc-libs
  glibc
  gtk3
)
makedepends=(
  git
  cargo
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
  echo "$(cargo pkgid | cut -d@ -f2).g$(git describe --always --exclude='*')"
}

build() {
  cd "${pkgname}"

  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release
}

package() {
  depends=(
    rustfmt
  )

  cd "${pkgname}"

  install -Dm755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
  install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}
