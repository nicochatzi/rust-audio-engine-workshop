#include <audio_engine/audio_engine.h>
#include <juce_audio_processors/juce_audio_processors.h>

//==============================================================================
class FloatParameter : public juce::AudioParameterFloat
{
public:
    //==============================================================================
    FloatParameter (juce::String parameterID,
                    juce::String parameterName,
                    float minValue,
                    float maxValue,
                    float defaultValue,
                    std::function<void (float)> valueChangedCallback)
        : juce::AudioParameterFloat (parameterID, parameterName, minValue, maxValue, defaultValue),
          onValueChanged (valueChangedCallback)
    {
    }

private:
    //==============================================================================
    void valueChanged (float newValue) override { onValueChanged (newValue); }

    std::function<void (float)> onValueChanged;
};

//==============================================================================
class AudioPluginAudioProcessor : public juce::AudioProcessor
{
public:
    //==============================================================================
    AudioPluginAudioProcessor()
        : juce::AudioProcessor (
            BusesProperties().withOutput ("Output", juce::AudioChannelSet::stereo(), true))
    {
        addParameter (new FloatParameter ("freq",
                                          "Frequency",
                                          64.f,
                                          880.f,
                                          110.f,
                                          [this] (float freq)
                                          { audio_engine::ffi_set_freq (&engine, freq); }));
        addParameter (new FloatParameter ("amp",
                                          "Amplitude",
                                          0.f,
                                          1.f,
                                          0.3f,
                                          [this] (float amp)
                                          { audio_engine::ffi_set_amp (&engine, amp); }));
    }

    ~AudioPluginAudioProcessor() override {}

    //==============================================================================
    void releaseResources() override {}

    bool isBusesLayoutSupported (const BusesLayout& layouts) const override
    {
        return layouts.getMainOutputChannelSet() == juce::AudioChannelSet::stereo();
    }

    //==============================================================================
    void prepareToPlay (double sampleRate, int samplesPerBlock) override
    {
        bufferSize = samplesPerBlock;
        numChannels = getTotalNumOutputChannels();

        audioBuffer.resize (bufferSize * numChannels);
        audio_engine::ffi_prepare (&engine, sampleRate, numChannels);
    }

    void processBlock (juce::AudioBuffer<float>& buffer, juce::MidiBuffer&) override
    {
        juce::ScopedNoDenormals noDenormals;

        for (auto i = getTotalNumInputChannels(); i < numChannels; ++i)
            buffer.clear (i, 0, buffer.getNumSamples());

        audio_engine::ffi_render (&engine, audioBuffer.data(), audioBuffer.size());

        deinterleaveAudioBufferInto (buffer);
    }

    void deinterleaveAudioBufferInto (juce::AudioBuffer<float>& buffer)
    {
        for (auto sample = 0; sample < bufferSize; ++sample)
            for (auto chan = 0; chan < numChannels; ++chan)
                *buffer.getWritePointer (chan, sample) = audioBuffer[(sample * numChannels) + chan];
    }

    using AudioProcessor::processBlock;

    //==============================================================================
    bool hasEditor() const override { return true; }
    juce::AudioProcessorEditor* createEditor() override
    {
        return new juce::GenericAudioProcessorEditor (*this);
    }

    //==============================================================================
    const juce::String getName() const override { return JucePlugin_Name; }

    bool acceptsMidi() const override { return false; }
    bool producesMidi() const override { return false; }
    bool isMidiEffect() const override { return false; }
    double getTailLengthSeconds() const override { return 0.0; }

    //==============================================================================
    int getNumPrograms() override { return 1; }
    int getCurrentProgram() override { return 0; }
    void setCurrentProgram (int) override {}
    const juce::String getProgramName (int) override {}
    void changeProgramName (int, const juce::String&) override {}

    //==============================================================================
    void getStateInformation (juce::MemoryBlock&) override {}
    void setStateInformation (const void*, int) override {}

private:
    //==============================================================================
    audio_engine::AudioEngine engine;
    std::vector<float> audioBuffer;
    int bufferSize;
    int numChannels;

    //==============================================================================
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (AudioPluginAudioProcessor)
};

//==============================================================================
juce::AudioProcessor* JUCE_CALLTYPE createPluginFilter() { return new AudioPluginAudioProcessor(); }
