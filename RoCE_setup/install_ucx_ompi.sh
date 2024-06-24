#!/bin/bash

# Variables for UCX and OMPI versions
UCX_VER="v1.12.1"  # Change this to the desired UCX version
OMPI_VER="v4.1.1"  # Change this to the desired OpenMPI version

# Directory for installation
INSTALL_DIR="$HOME"

# Function to clone, build, and install UCX
install_ucx() {
    echo "Installing UCX..."
    cd $INSTALL_DIR
    git clone --recursive -b ${UCX_VER} https://github.com/openucx/ucx.git
    cd ucx
    ./autogen.sh
    mkdir ucx_install build
    cd build
    ../contrib/configure-release --disable-debug --disable-assertions --disable-params-check \
    --with-rocm=/opt/rocm --with-rc --with-ud --with-dc --with-dm --with-ib-hw-tm \
    --prefix=$INSTALL_DIR/ucx/ucx_install --disable-log
    make -j $(nproc)
    make -j $(nproc) install

    # Verify UCX installation
    $INSTALL_DIR/ucx/ucx_install/bin/ucx_info -d
}

# Function to clone, build, and install OpenMPI
install_ompi() {
    echo "Installing OpenMPI..."
    # Ensure flex is installed
    sudo apt install -y flex

    cd $INSTALL_DIR
    git clone --recursive -b $OMPI_VER https://github.com/open-mpi/ompi.git
    cd ompi
    ./autogen.pl
    mkdir build ompi_install
    cd build
    ../configure --prefix=$INSTALL_DIR/ompi/ompi_install --with-ucx=$INSTALL_DIR/ucx/ucx_install \
    --enable-mca-no-build=btl-uct
    make -j $(nproc)
    make -j $(nproc) install

    # Verify OpenMPI installation
    $INSTALL_DIR/ompi/ompi_install/bin/ompi_info | grep Configure
}

# Execute the functions
install_ucx
install_ompi

echo "UCX and OpenMPI installation completed."

