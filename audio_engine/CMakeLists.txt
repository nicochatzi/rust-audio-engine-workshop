cmake_minimum_required(VERSION 3.15)

set(AUDIO_ENGINE_HEADERS
    include/audio_engine/audio_engine.h
)

file(GLOB_RECURSE AUDIO_ENGINE_SOURCES CONFIGURE_DEPENDS
    ${CMAKE_CURRENT_SOURCE_DIR}/src/*.rs
)

source_group(TREE ${CMAKE_CURRENT_SOURCE_DIR}
    FILES
        ${AUDIO_ENGINE_SOURCES}
        ${AUDIO_ENGINE_HEADERS}
)

if(CMAKE_BUILD_TYPE EQUAL Debug)
    set(TARGET_DIR debug)
    set(CARGO_BUILD_CONFIG)
else()
    set(TARGET_DIR release)
    set(CARGO_BUILD_CONFIG --release)
endif()

set(AUDIO_ENGINE_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/target/${TARGET_DIR})

if(CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
    set(AUDIO_ENGINE_LIB ${AUDIO_ENGINE_LIB_DIR}/audio_engine.lib)
else()
    set(AUDIO_ENGINE_LIB ${AUDIO_ENGINE_LIB_DIR}/libaudio_engine.a)
endif()

# When building using this CMakeLists, we are assuming
# the consumer is on a desktop platform and is supplying
# the audio host so we don't have the cpal feature.
set(CARGO_BUILD_FLAGS --lib --no-default-features --features ffi)

# Some environments might not have cargo in their PATH
set(CARGO_BIN "$ENV{HOME}/.cargo/bin/cargo")

add_custom_target(audio_engine
    COMMAND ${CARGO_BIN} build ${CARGO_BUILD_FLAGS} ${CARGO_BUILD_CONFIG}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    DEPENDS ${AUDIO_ENGINE_SOURCES} ${AUDIO_ENGINE_HEADERS}
    SOURCES ${AUDIO_ENGINE_SOURCES}
)

add_library(audio_engine_lib INTERFACE)

target_link_libraries(audio_engine_lib INTERFACE ${AUDIO_ENGINE_LIB})

target_include_directories(audio_engine_lib
    INTERFACE
        $<INSTALL_INTERFACE:include>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
)