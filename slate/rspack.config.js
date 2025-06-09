// @ts-check

/** @type {import('@rspack/cli').Configuration} */
const config = {
  entry: {
    main: "./src/index.tsx",
  },
  optimization: {
    // minimize: false
  },
  experiments: {
    outputModule: true,
  },
  output: {
    module: true,
    library: {
      type: "module",
      //   type: "umd",
      // name: "MyLibrary",
      //   // type: "var",
      // export: "default",
    },
  },
  resolve: {
    extensions: [".js", ".json", ".wasm", ".ts", ".tsx"],
  },
  module: {
    rules: [
      {
        test: /\.jsx$/,
        use: {
          loader: "builtin:swc-loader",
          options: {
            jsc: {
              parser: {
                syntax: "ecmascript",
                jsx: true,
              },
            },
          },
        },
        type: "javascript/auto",
      },
      {
        test: /\.tsx$/,
        use: {
          loader: "builtin:swc-loader",
          options: {
            jsc: {
              parser: {
                syntax: "typescript",
                tsx: true,
              },
            },
          },
        },
        type: "javascript/auto",
      },
    ],
  },
};
module.exports = config;
