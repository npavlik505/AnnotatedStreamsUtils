# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.28

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build

# Include any dependencies generated for this target.
include src/CMakeFiles/H5detect.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include src/CMakeFiles/H5detect.dir/compiler_depend.make

# Include the progress variables for this target.
include src/CMakeFiles/H5detect.dir/progress.make

# Include the compile flags for this target's objects.
include src/CMakeFiles/H5detect.dir/flags.make

src/CMakeFiles/H5detect.dir/H5detect.c.o: src/CMakeFiles/H5detect.dir/flags.make
src/CMakeFiles/H5detect.dir/H5detect.c.o: /home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5detect.c
src/CMakeFiles/H5detect.dir/H5detect.c.o: src/CMakeFiles/H5detect.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object src/CMakeFiles/H5detect.dir/H5detect.c.o"
	cd /home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT src/CMakeFiles/H5detect.dir/H5detect.c.o -MF CMakeFiles/H5detect.dir/H5detect.c.o.d -o CMakeFiles/H5detect.dir/H5detect.c.o -c /home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5detect.c

src/CMakeFiles/H5detect.dir/H5detect.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing C source to CMakeFiles/H5detect.dir/H5detect.c.i"
	cd /home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5detect.c > CMakeFiles/H5detect.dir/H5detect.c.i

src/CMakeFiles/H5detect.dir/H5detect.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling C source to assembly CMakeFiles/H5detect.dir/H5detect.c.s"
	cd /home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src/H5detect.c -o CMakeFiles/H5detect.dir/H5detect.c.s

# Object files for target H5detect
H5detect_OBJECTS = \
"CMakeFiles/H5detect.dir/H5detect.c.o"

# External object files for target H5detect
H5detect_EXTERNAL_OBJECTS =

bin/H5detect: src/CMakeFiles/H5detect.dir/H5detect.c.o
bin/H5detect: src/CMakeFiles/H5detect.dir/build.make
bin/H5detect: src/CMakeFiles/H5detect.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking C executable ../bin/H5detect"
	cd /home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/src && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/H5detect.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
src/CMakeFiles/H5detect.dir/build: bin/H5detect
.PHONY : src/CMakeFiles/H5detect.dir/build

src/CMakeFiles/H5detect.dir/clean:
	cd /home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/src && $(CMAKE_COMMAND) -P CMakeFiles/H5detect.dir/cmake_clean.cmake
.PHONY : src/CMakeFiles/H5detect.dir/clean

src/CMakeFiles/H5detect.dir/depend:
	cd /home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5 /home/nate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hdf5-src-0.8.1/ext/hdf5/src /home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build /home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/src /home/nate/Desktop/StreamsUtilsNP/target/debug/build/hdf5-src-794a3d6b46e0bae9/out/build/src/CMakeFiles/H5detect.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : src/CMakeFiles/H5detect.dir/depend

