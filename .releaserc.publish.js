const base = require("./.releaserc.base.js");

const { plugins: pluginsExtracted, rest } = base;

module.exports = {
  "plugins": [
    ...pluginsExtracted, ,
    [
      "@semantic-release/changelog",
      {
        "changelogTitle": "Changelog",
        "changelogFile": "CHANGELOG.md"
      }
    ],
    "@semantic-release/github"
  ],
  ...rest
}
