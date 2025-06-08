cmake_minimum_required(VERSION 3.16)

project(BaseLine LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(
BaseLine main.cpp)

add_subdirectory(Classes)

include(GNUInstallDirs)
install(TARGETS BaseLine
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
