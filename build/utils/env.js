const fs = require('fs');

const dotenv = require('dotenv');

const paths = require('./paths');

function getEnvVars(env) {
  const envFiles = ['.env.lab'];
  envFiles.push(`.env.${env}`);

  const envVars = {};

  envFiles.forEach((file) => {
    const fullPath = paths.resolve(file);
    if (fs.existsSync(fullPath)) {
      const result = dotenv.parse(fs.readFileSync(fullPath));
      Object.assign(envVars, result);
    }
  });

  return envVars;
}

module.exports = getEnvVars;
