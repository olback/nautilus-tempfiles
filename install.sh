#!/bin/sh

if [[ $# -eq 1 && $1 == "uninstall" ]]; then

    # Uninstall
    rm /usr/lib/nautilus/extensions-3.0/libtempfiles-upload.so

    echo "Uninstalled"

else

    # Install
    cp target/release/libtempfiles_nautilus.so /usr/lib/nautilus/extensions-3.0/libtempfiles-upload.so

    echo "Installed"

fi
