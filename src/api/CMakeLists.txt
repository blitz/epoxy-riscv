cmake_minimum_required(VERSION 3.12)

project("epoxy-api" VERSION 1.0.0
  DESCRIPTION "A C++ library providing basic headers to interact with the Epoxy kernel.")

include(GNUInstallDirs)

add_library(${PROJECT_NAME} INTERFACE)

target_include_directories(
  ${PROJECT_NAME}
  INTERFACE $<BUILD_INTERFACE:${${PROJECT_NAME}_SOURCE_DIR}/include>
            $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>)

target_compile_features(${PROJECT_NAME} INTERFACE cxx_std_17)

install(
  DIRECTORY ${PROJECT_SOURCE_DIR}/include/${PROJECT_NAME}
  DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
