cmake_minimum_required(VERSION 3.12)

project("epoxy-virtio-net" VERSION 1.0.0
  DESCRIPTION "Run a simple HTTP server via a virtio-net device and uIP.")

include(GNUInstallDirs)

find_package(pprintpp REQUIRED)
find_package(range-v3 REQUIRED)

add_executable(${PROJECT_NAME}
  src/main.cpp

  # uIP
  src/uip/psock.c
  src/uip/timer.c
  src/uip/uip_arp.c
  src/uip/uip.c
  src/uip/uiplib.c

  # Webserver
  src/apps/webserver/httpd.c
  src/apps/webserver/httpd-cgi.c
  src/apps/webserver/httpd-fs.c
  src/apps/webserver/http-strings.c
  )
target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_17)

target_include_directories(
  ${PROJECT_NAME}
  PRIVATE include src/uip src/apps/webserver
  )

target_link_libraries(
  ${PROJECT_NAME}
  ${PPRINTPP_LIBRARIES} ${RANGE-V3_LIBRARIES})

set_property(TARGET ${PROJECT_NAME} PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
set_target_properties(${PROJECT_NAME} PROPERTIES LINK_FLAGS_RELEASE -s)

install(
  TARGETS ${PROJECT_NAME}
  DESTINATION ${CMAKE_INSTALL_BINDIR})
