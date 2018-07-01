# `before_deploy` phase: here we package the build artifacts

set -ex

mktempd() {
    echo $(mktemp -d 2>/dev/null || mktemp -d -t tmp)
}

host() {
    case "$TRAVIS_OS_NAME" in
        linux)
            echo x86_64-unknown-linux-gnu
            ;;
        osx)
            echo x86_64-apple-darwin
            ;;
    esac
}

gcc_prefix() {
    case "$TARGET" in
        aarch64-unknown-linux-gnu)
            echo aarch64-linux-gnu-
            ;;
        arm*-gnueabihf)
            echo arm-linux-gnueabihf-
            ;;
        *-musl)
            echo musl-
            ;;
        *)
            return
            ;;
    esac
}

dobin() {
    [ -z $MAKE_DEB ] && die 'dobin: $MAKE_DEB not set'
    [ $# -lt 1 ] && die "dobin: at least one argument needed"

    local f prefix=$(gcc_prefix)
    for f in "$@"; do
        install -m0755 $f $dtd/debian/usr/bin/
        ${prefix}strip -s $dtd/debian/usr/bin/$(basename $f)
    done
}

architecture() {
    case $1 in
        x86_64-unknown-linux-gnu|x86_64-unknown-linux-musl)
            echo amd64
            ;;
        i686-unknown-linux-gnu|i686-unknown-linux-musl)
            echo i386
            ;;
        arm*-unknown-linux-gnueabihf)
            echo armhf
            ;;
        *)
            die "architecture: unexpected target $TARGET"
            ;;
    esac
}

# Generate artifacts for release
mk_artifacts() {
    cargo build --target $TARGET --release
}

mk_tarball() {
    # create a "staging" directory
    local td=$(mktempd)
    local out_dir=$(pwd)

    # TODO update this part to copy the artifacts that make sense for your project
    # NOTE All Cargo build artifacts will be under the 'target/$TARGET/{debug,release}'
    cp target/$TARGET/release/hello $td

    pushd $td

    # release tarball will look like 'rust-everywhere-v1.2.3-x86_64-unknown-linux-gnu.tar.gz'
    tar czf $out_dir/${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}.tar.gz *

    popd
    rm -r $td
}

main() {
    mk_artifacts
    mk_tarball
}

main