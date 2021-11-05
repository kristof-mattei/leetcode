const base = require("./.releaserc.base.js");

const { plugins: pluginsExtracted, rest } = base;

module.exports = {
  "plugins": [
    ...pluginsExtracted,
    [
      "@semantic-release/exec",
      {
        "verifyReleaseCmd": "echo ${nextRelease.version} > NEXTVERSION"
      }
    ]
  ],
  ...rest
}
