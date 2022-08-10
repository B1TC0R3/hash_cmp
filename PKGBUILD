# Maintainer: Thomas Gingele (https://github.com/B1TC0R3)
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=hash-cmp-bin
pkgver=4.0.0
pkgrel=1
pkgdesc="Validates the hash of a file against a known hash"
url="https://github.com/B1TC0R3/hash-cmp"
license=("GPL-3.0")
arch=("x86_64")
provides=("hash-cmp")
conflicts=("hash-cmp")
source=("https://github.com/B1TC0R3/hash-cmp/releases/download/v$pkgver/hash-cmp-$pkgver-x86_64.tar.gz")
sha256sums=("4bc22121ac37f999e5ae48ee77bcb1d65b28698fb91843b1e1e8bf9365c17f73")

package() {
    install -Dm755 hash-cmp -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
