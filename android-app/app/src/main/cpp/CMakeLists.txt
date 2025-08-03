cmake_minimum_required(VERSION 3.18.1)

project("solana_sos")

# Set the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add the Rust library
add_library(solana_sos SHARED IMPORTED)
set_target_properties(solana_sos PROPERTIES IMPORTED_LOCATION
    ${CMAKE_CURRENT_SOURCE_DIR}/../jniLibs/${ANDROID_ABI}/libsolana_sos.so)

# Create the JNI wrapper library
add_library(solana_sos_jni SHARED
    solana_sos_jni.cpp)

# Link against the Rust library
target_link_libraries(solana_sos_jni
    solana_sos
    android
    log)

# Include directories
target_include_directories(solana_sos_jni PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include)

# Compiler flags
target_compile_options(solana_sos_jni PRIVATE
    -Wall
    -Wextra
    -Werror
    -fexceptions
    -frtti) 