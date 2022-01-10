cmake_minimum_required(VERSION 3.15)

project(AUDIO_PLUGIN_EXAMPLE
    VERSION 0.0.1
    LANGUAGES CXX
)

add_subdirectory(vst_plugin/lib/juce)
add_subdirectory(vst_plugin/lib/googletest)
add_subdirectory(audio_engine)

juce_add_plugin(AudioPluginExample
    PLUGIN_MANUFACTURER_CODE    Rite
    PLUGIN_CODE                 Dem0
    FORMATS                     AU VST3 Standalone
    PRODUCT_NAME                "Audio Plugin Example"
)

target_sources(AudioPluginExample
    PRIVATE
        vst_plugin/src/main.cpp
)

target_compile_definitions(AudioPluginExample
    PUBLIC
        JUCE_WEB_BROWSER=0
        JUCE_USE_CURL=0
        JUCE_VST3_CAN_REPLACE_VST2=0
)

add_dependencies(AudioPluginExample audio_engine)

target_link_libraries(AudioPluginExample
    PRIVATE
        juce::juce_audio_utils
    PUBLIC
        juce::juce_recommended_config_flags
        juce::juce_recommended_lto_flags
        juce::juce_recommended_warning_flags
        audio_engine_lib
)
