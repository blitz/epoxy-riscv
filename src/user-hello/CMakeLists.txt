cmake_minimum_required(VERSION 3.12)

project("epoxy-hello" VERSION 1.0.0
  DESCRIPTION "Greet the world. Programmer-style.")

include(GNUInstallDirs)

set(TARGET_BIN_NAME ${PROJECT_NAME} CACHE STRING "The name with with the binary is installed")

add_executable(${PROJECT_NAME} src/main.cpp)
set_target_properties(${PROJECT_NAME} PROPERTIES OUTPUT_NAME ${TARGET_BIN_NAME})

target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_17)

set_property(TARGET ${PROJECT_NAME} PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
set_target_properties(${PROJECT_NAME} PROPERTIES LINK_FLAGS_RELEASE -s)

install(
  TARGETS ${PROJECT_NAME}
  DESTINATION ${CMAKE_INSTALL_BINDIR}
  RENAME ${TARGET_BIN_NAME})
