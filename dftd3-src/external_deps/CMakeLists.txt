cmake_minimum_required(VERSION 3.15)
project(dep-simple-dftd3)

include(ExternalProject)

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE RELWITHDEBINFO)
endif()

set(DFTD3_SRC "" CACHE PATH "User specified directory of source code (s-dftd3)")
if(NOT DFTD3_SRC AND (NOT $ENV{DFTD3_SRC} EQUAL ""))
    set(DFTD3_SRC $ENV{DFTD3_SRC})
endif()

if(DFTD3_SRC)
    message(STATUS "User specified directory of source code (s-dftd3): ${DFTD3_SRC}")
else()
    set(DFTD3_SRC https://github.com/dftd3/simple-dftd3.git)
    message(STATUS "Download source code (s-dftd3): ${DFTD3_SRC}")
endif()

set(DFTD3_VER "" CACHE PATH "User specified version of library (git tag) (s-dftd3)")
if(NOT DFTD3_VER AND (NOT $ENV{DFTD3_VER} EQUAL ""))
    set(DFTD3_VER $ENV{DFTD3_VER})
endif()

if(DFTD3_VER)
    message(STATUS "User specified version of library (s-dftd3): ${DFTD3_VER}")
else()
    set(DFTD3_VER v1.2.1)
    message(STATUS "Download version of library (s-dftd3): ${DFTD3_VER}")
endif()

ExternalProject_Add(s-dftd3
    GIT_REPOSITORY ${DFTD3_SRC}
    GIT_TAG ${DFTD3_VER}
    GIT_SHALLOW TRUE
    GIT_PROGRESS TRUE
    PREFIX ${PROJECT_BINARY_DIR}/deps
    CMAKE_ARGS
        -GNinja
        -DCMAKE_BUILD_TYPE=RELEASE
        -DBUILD_SHARED_LIBS=${BUILD_SHARED_LIBS}
        -DCMAKE_INSTALL_PREFIX=${PROJECT_BINARY_DIR}/deps
)

install(DIRECTORY ${PROJECT_BINARY_DIR}/deps/lib DESTINATION ${CMAKE_INSTALL_PREFIX})