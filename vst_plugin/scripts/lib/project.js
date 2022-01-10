const path = require('path');
const { execSync } = require('child_process');

const isOnMac = process.platform === 'darwin';
const buildConfig = process.argv[2] === 'Debug' ? 'Debug' : 'Release';

const dirs = {
  root: path.join(__dirname, '..', '..', '..'),
  build: path.join(__dirname, '..', '..', 'build'),
};

const cmake = (args) =>
  execSync(`cmake ${args.join(' ')}`, {
    stdio: 'inherit',
  });

module.exports = {
  dirs,
  isOnMac,
  buildConfig,
  cmake,
  build: () =>
    cmake(['--build', dirs.build, '--parallel', '8', '--config', buildConfig]),
  generate: () =>
    cmake([
      '-S',
      dirs.root,
      '-B',
      dirs.build,
      '-G',
      isOnMac ? 'Xcode' : 'Visual Studio 16 2019',
    ]),
  open: (projectName) =>
    execSync(
      `${isOnMac ? 'open' : 'explorer'} ${path.join(
        dirs.build,
        projectName + (isOnMac ? '.xcodeproj' : '.sln')
      )}`
    ),
};
