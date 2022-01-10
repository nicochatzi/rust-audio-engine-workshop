# Audio Plugin and Standalone app with JUCE

This is a JUCE-based wrapper around the audio engine.

## Commands

This project uses CMake as a build configuration tool and node scripts
as a task runner.

```bash
$ npm run clean     # Removes the build directory
$ npm run generate  # Generate the project build system
$ npm run build     # Build the project, defaults to Release
$ npm run build Debug
$ npm test          # Run all the tests
$ npm start         # Open the project in an IDE (XCode or Visual Studio)
                    # requires calling npm run generate (or build) once first.
```
