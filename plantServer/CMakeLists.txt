
cmake_minimum_required(VERSION 3.5)

project(PlantServer)

file(GLOB SOURCES 
    "src/*.cpp"
    "*.cpp"
)

set(CMAKE_CXX_STANDARD 17)
if(BUILD_ARM)
    set(CMAKE_C_COMPILER /usr/bin/arm-linux-gnueabihf-gcc-5)
    set(CMAKE_CXX_COMPILER arm-linux-gnueabihf-g++-5)
else()
    add_subdirectory(tests)
endif(BUILD_ARM)

add_executable(${PROJECT_NAME} main.cpp ${SOURCES})
target_link_libraries(${PROJECT_NAME} wiringPi)
target_link_libraries(${PROJECT_NAME} wiringPiDev)
target_link_libraries(${PROJECT_NAME} pthread)