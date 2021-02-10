cmake_minimum_required(VERSION 3.12)

project("epoxy-blink" VERSION 1.0.0
  DESCRIPTION "Blink the LEDs of the ULX3S board.")

set(RESOURCE_HEADER "${CMAKE_CURRENT_SOURCE_DIR}/src/default-resources.hpp" CACHE FILEPATH "The Epoxy resource header")
include(GNUInstallDirs)

set(TARGET_BIN_NAME ${PROJECT_NAME} CACHE STRING "The name with with the binary is installed")

add_custom_command(
  OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/src/resources.hpp
  COMMAND cp -v ${RESOURCE_HEADER} ${CMAKE_CURRENT_BINARY_DIR}/src/resources.hpp
  )

add_executable(${PROJECT_NAME}
  src/main.cpp
  ${CMAKE_CURRENT_BINARY_DIR}/src/resources.hpp)
set_target_properties(${PROJECT_NAME} PROPERTIES OUTPUT_NAME ${TARGET_BIN_NAME})

target_include_directories(${PROJECT_NAME}
          INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/src
          PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/src
          )

target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_17)

set_property(TARGET ${PROJECT_NAME} PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
set_target_properties(${PROJECT_NAME} PROPERTIES LINK_FLAGS_RELEASE -s)

install(
  TARGETS ${PROJECT_NAME}
  DESTINATION ${CMAKE_INSTALL_BINDIR}
  RENAME ${TARGET_BIN_NAME})
