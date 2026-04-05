cmake_minimum_required(VERSION 4.2)
project(cpp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(ADLX_ROOT ${CMAKE_SOURCE_DIR}/../libs/ADLX)

set(SOURCES
        ADLXWrapper.cpp
        ${ADLX_ROOT}/SDK/ADLXHelper/Windows/Cpp/ADLXHelper.cpp
        ${ADLX_ROOT}/SDK/Platform/Windows/WinAPIs.cpp
)

add_executable(cpp ${SOURCES})

target_include_directories(cpp PRIVATE
        ${ADLX_ROOT}/SDK/Include
        ${ADLX_ROOT}/SDK/ADLXHelper/Windows/Cpp
        ${ADLX_ROOT}/SDK
)

if (MSVC)
    target_compile_options(cpp PRIVATE /EHsc)
endif ()
