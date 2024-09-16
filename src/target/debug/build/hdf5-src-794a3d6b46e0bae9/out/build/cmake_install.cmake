# Install script for directory: /home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5

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

if(CMAKE_INSTALL_COMPONENT STREQUAL "configinstall" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/share/cmake/hdf5/hdf5-targets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/share/cmake/hdf5/hdf5-targets.cmake"
         "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/CMakeFiles/Export/f482e566b80ee08e30cf8712f7d31e33/hdf5-targets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/share/cmake/hdf5/hdf5-targets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/share/cmake/hdf5/hdf5-targets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/cmake/hdf5" TYPE FILE FILES "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/CMakeFiles/Export/f482e566b80ee08e30cf8712f7d31e33/hdf5-targets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/cmake/hdf5" TYPE FILE FILES "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/CMakeFiles/Export/f482e566b80ee08e30cf8712f7d31e33/hdf5-targets-debug.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "configinstall" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/cmake/hdf5" TYPE FILE FILES "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/CMakeFiles/hdf5-config.cmake")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "configinstall" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/cmake/hdf5" TYPE FILE FILES "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/CMakeFiles/hdf5-config-version.cmake")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "libraries" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE FILE FILES "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/libhdf5.settings")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "hdfdocuments" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share" TYPE FILE FILES "/home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/COPYING")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/src/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
