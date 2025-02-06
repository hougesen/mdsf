// @ts-check

import typescriptEslint from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";

export default [
  { files: ["**/*.ts"] },
  {
    languageOptions: {
      ecmaVersion: 2022,
      parser: tsParser,
      sourceType: "module",
    },
    plugins: { "@typescript-eslint": typescriptEslint },
  },
  {
    rules: {
      "@typescript-eslint/array-type": ["error", { default: "array-simple" }],
      "@typescript-eslint/consistent-indexed-object-style": "error",
      "@typescript-eslint/no-empty-object-type": [
        "error",
        { allowInterfaces: "with-single-extends" },
      ],
      "@typescript-eslint/no-unused-vars": [
        "error",
        {
          args: "all",
          argsIgnorePattern: "^_",
          caughtErrors: "all",
          caughtErrorsIgnorePattern: "^_",
          destructuredArrayIgnorePattern: "^_",
          ignoreRestSiblings: true,
          varsIgnorePattern: "^_",
        },
      ],
      curly: "warn",
      eqeqeq: "warn",
      "no-duplicate-imports": "error",
      "no-throw-literal": "warn",
      "object-shorthand": "error",
      semi: "warn",
    },
  },
  { ignores: ["dist", "./dist/*", "out"] },
];
