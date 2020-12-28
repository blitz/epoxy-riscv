cmake_minimum_required(VERSION 3.12)

project("epoxy-hello" VERSION 1.0.0
  DESCRIPTION "Great the world programmer-style.")

include(GNUInstallDirs)

add_executable(${PROJECT_NAME} src/main.cpp)
target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_17)

install(
  TARGETS ${PROJECT_NAME}
  DESTINATION ${CMAKE_INSTALL_BINDIR})
