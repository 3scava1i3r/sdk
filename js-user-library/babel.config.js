module.exports = {
  ignore: [
    "./src/IDL-ts",
  ],
  presets: [
    [
      "@babel/preset-env",
      {
        corejs: { version: 3, proposals: true },
        targets: {
          browsers: "defaults",
          node: "current",
        },
        "useBuiltIns": "usage",
      },
    ],
    [
      "@babel/preset-typescript",
    ],
  ],
};
