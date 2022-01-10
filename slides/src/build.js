const path = require('path');
const { execSync } = require('child_process');
const fs = require('fs');

const dirs = {
  input: path.join(__dirname, '../', 'slides'),
  output: path.join(__dirname, '../', 'out'),
};

fs.rmSync(dirs.output, { recursive: true, force: true });
fs.readdir(dirs.input, (e, files) =>
  files.forEach((file) =>
    execSync(`marp ${dirs.input}/${file} -o ${dirs.output}/${file}.pdf`)
  )
);
