cmake_minimum_required(VERSION 3.12)
project(libcxx-demo LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(cxx-demo STATIC lib.cc lib.h)

install(TARGETS cxx-demo
    LIBRARY DESTINATION "${CMAKE_INSTALL_LIBDIR}"
)
