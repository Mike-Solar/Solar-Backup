cmake_minimum_required(VERSION 3.30)
project(Solar_Backup)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

set(CMAKE_PREFIX_PATH "/opt/Qt/6.8.2/gcc_64")

find_package(Qt6 COMPONENTS
  Core
  Gui
  Widgets
  REQUIRED)

add_executable(Solar_Backup main.cpp)
target_link_libraries(Solar_Backup
  Qt::Core
  Qt::Gui
  Qt::Widgets
)

