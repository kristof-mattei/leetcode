const base = require("./.releaserc.js");

const { plugins: pluginsExtracted, rest } = base;

module.exports =
{
  "plugins": [
    ...pluginsExtracted,
    "@semantic-release/github"
  ],
  ...rest
}
