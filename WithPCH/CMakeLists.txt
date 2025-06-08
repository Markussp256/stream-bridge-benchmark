cmake_minimum_required(VERSION 3.16)

project(WithPCH LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


add_executable(WithPCH main.cpp
    precompiled.h)

target_precompile_headers(WithPCH PRIVATE precompiled.h)
add_subdirectory(Classes)

target_include_directories(Classes PUBLIC ${CMAKE_SOURCE_DIR})

include(GNUInstallDirs)
install(TARGETS WithPCH
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
