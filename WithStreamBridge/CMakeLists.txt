cmake_minimum_required(VERSION 3.16)

project(WithStreamBridge LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(FetchContent)

# Fetch the external GitHub library
FetchContent_Declare(
  stream-bridge
  GIT_REPOSITORY https://github.com/Markussp256/stream-bridge.git
  GIT_TAG main
)

FetchContent_MakeAvailable(stream-bridge)

# Add the Classes submodule (assuming 'Classes' is a subdirectory in your project)
add_subdirectory(Classes)

# Add your main app executable
add_executable(WithStreamBridge
    main.cpp
)

# Link against the Classes library and stream-bridge
target_link_libraries(WithStreamBridge
    PRIVATE
        Classes
        stream-bridge
)

target_include_directories(WithStreamBridge PRIVATE
    $<TARGET_PROPERTY:stream-bridge,INTERFACE_INCLUDE_DIRECTORIES>
)

# Optional install
include(GNUInstallDirs)
install(TARGETS WithStreamBridge
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
