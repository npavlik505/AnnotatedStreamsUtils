# Install script for directory: /home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "headers" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/hdf5.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5api_adpt.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5public.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Apublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5ACpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Cpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Dpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Epubgen.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Epublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Fpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDcore.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDdirect.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDfamily.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDhdfs.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDlog.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDmirror.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDmpi.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDmpio.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDmulti.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDros3.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDs3comms.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDsec2.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDsplitter.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDstdio.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5FDwindows.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Gpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Ipublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Lpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5MMpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Opublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Ppublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5PLextern.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5PLpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Rpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Spublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Tpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Zpublic.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5Epubgen.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5version.h"
    "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5overflow.h"
    "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/H5pubconf.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "libraries" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/bin/libhdf5_debug.a")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "libraries" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/CMakeFiles/hdf5-1.10.7.pc")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "libraries" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE FILE PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE FILES "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/CMakeFiles/h5cc")
endif()

