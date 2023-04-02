cmake_minimum_required(VERSION 3.23)
project(cpp_lib)
# example library with sources and headers located at \${MY_LIB_SOURCE_DIR}
add_library(${PROJECT_NAME} SHARED)

set_target_properties(${PROJECT_NAME} PROPERTIES CXX_STANDARD 17)

target_include_directories(${PROJECT_NAME} PUBLIC include)

# source files get added as before
target_sources(${PROJECT_NAME} PUBLIC src/functions.cpp)

include(GenerateExportHeader)
generate_export_header(${PROJECT_NAME} EXPORT_FILE_NAME ${CMAKE_BINARY_DIR}/exports/${PROJECT_NAME}_export.h)

target_include_directories(${PROJECT_NAME} PUBLIC ${CMAKE_BINARY_DIR}/exports)

install(TARGETS ${PROJECT_NAME}
  EXPORT ${PROJECT_NAME}-targets
  LIBRARY DESTINATION lib
  ARCHIVE DESTINATION lib
  RUNTIME DESTINATION bin
  INCLUDES DESTINATION include
)
install(DIRECTORY include/ DESTINATION include)
install(DIRECTORY ${CMAKE_BINARY_DIR}/exports/ DESTINATION include)
