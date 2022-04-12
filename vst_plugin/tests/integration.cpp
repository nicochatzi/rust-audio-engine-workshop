#include <gtest/gtest.h>
#include <audio_engine/audio_engine.h>

//==================================================================================================

int main (int argc, char** argv)
{
    ::testing::InitGoogleTest (&argc, argv);
    return RUN_ALL_TESTS();
}

//==================================================================================================

TEST (RustIntegration, can_prepare_an_audio_engine)
{
    auto SAMPLE_RATE = 1000;
    auto NUM_CHANNELS = 2;

    audio_engine::AudioEngine engine;
    audio_engine::ffi_prepare (&engine, SAMPLE_RATE, NUM_CHANNELS);

    ASSERT_EQ (engine.sample_time, 1.f / SAMPLE_RATE);
    ASSERT_EQ (engine.num_channels, NUM_CHANNELS);
}