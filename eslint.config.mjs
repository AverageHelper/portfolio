// @ts-check
import fileProgress from "eslint-plugin-file-progress";
import prettier from "eslint-plugin-prettier";
import unicorn from "eslint-plugin-unicorn";
import promise from "eslint-plugin-promise";
import * as _import from "eslint-plugin-import";
import jsxA11Y from "eslint-plugin-jsx-a11y";
import typescriptEslint from "typescript-eslint";
import { fixupPluginRules } from "@eslint/compat";
import parser from "astro-eslint-parser";
import path from "node:path";
import { fileURLToPath } from "node:url";
import js from "@eslint/js";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const compat = new FlatCompat({
	baseDirectory: __dirname,
	recommendedConfig: js.configs.recommended,
	allConfig: js.configs.all,
});

// eslint-disable-next-line import/no-default-export
export default [
	{
		ignores: ["**/lib", "**/node_modules", "**/functions", "**/vendor", ".astro"],
	},
	js.configs.recommended,
	...compat.extends(
		"strictest/eslint",
		"strictest/promise",
		"strictest/typescript-eslint",
		// "strictest/unicorn", // Added rules manually until this is updated for new unicorn
		"plugin:prettier/recommended",
		"plugin:astro/recommended",
	),
	...typescriptEslint.configs.recommended,
	...typescriptEslint.configs.recommendedTypeChecked,
	jsxA11Y.flatConfigs.recommended,
	unicorn.configs.recommended,
	{
		plugins: {
			"file-progress": fileProgress,
			prettier,
			promise,
			import: fixupPluginRules(_import),
			"@typescript-eslint": typescriptEslint.plugin,
		},

		languageOptions: {
			parser: typescriptEslint.parser,
			parserOptions: {
				projectService: {
					projectService: true,
					allowDefaultProject: ["*.js", "*.mjs"], // allows parsing the eslint config
				},
				tsconfigRootDir: import.meta.dirname,
			},
		},

		rules: {
			"file-progress/activate": 1,
			"prettier/prettier": "warn",
			"no-constant-condition": "warn",
			"no-console": "warn",
			"no-dupe-else-if": "warn",
			"consistent-return": "off",
			"no-duplicate-imports": "off",
			"import/no-duplicates": "warn",
			"import/no-default-export": "error",
			"import/extensions": ["error", "ignorePackages"],
			"@typescript-eslint/no-deprecated": "warn",
			"@typescript-eslint/no-empty-interface": "off",
			"@typescript-eslint/require-await": "warn",
			"@typescript-eslint/no-inferrable-types": "off",
			"@typescript-eslint/no-unused-vars": ["warn", { varsIgnorePattern: "Props" }],

			// From https://github.com/astoilkov/eslint-config-strictest/blob/b7e92cbb546eadb6f0c23160b8b857f77f68ce63/unicorn.js (until strictest is updated against new unicorn)
			"unicorn/prefer-set-has": "error",
			"unicorn/no-object-as-default-parameter": "error", // undocumented
			"unicorn/prefer-number-properties": "error",
			"unicorn/prefer-optional-catch-binding": "error",
			"unicorn/prefer-array-find": "error", // undocumented
			"unicorn/no-instanceof-builtins": "error", // Updated from no-array-instanceof
			"unicorn/no-abusive-eslint-disable": "error",
			"unicorn/no-console-spaces": "error",
			// "unicorn/no-array-callback-reference": "error", // Updated from no-fn-reference-in-iterator, overridden below
			"unicorn/prefer-dom-node-text-content": "error", // Updated from prefer-text-content
			"unicorn/prefer-add-event-listener": "error",
			"unicorn/prefer-type-error": "error",
			"unicorn/prefer-keyboard-event-key": "error", // Updated from prefer-event-key
			"unicorn/throw-new-error": "error",
			// "unicorn/catch-error-name": ["error", { name: "err" }], // Overridden later
			"unicorn/no-unused-properties": "error",
			"unicorn/no-zero-fractions": "error",
			"unicorn/prefer-modern-dom-apis": "error",
			"unicorn/prefer-dom-node-append": "error", // Updated from prefer-node-append
			"unicorn/prefer-dom-node-remove": "error", // Updated from prefer-node-remove
			"unicorn/prefer-query-selector": "error",
			"unicorn/better-regex": "error",
			"unicorn/error-message": "error",
			"unicorn/escape-case": "error",
			"unicorn/explicit-length-check": ["error", { "non-zero": "greater-than" }],
			"unicorn/new-for-builtins": "error",
			"unicorn/filename-case": ["error", { cases: { camelCase: true, pascalCase: true } }],
			"unicorn/no-unreadable-array-destructuring": "error",
			"unicorn/prefer-array-flat-map": "error", // Updated from prefer-flat-map
			"unicorn/prefer-negative-index": "error",
			"unicorn/prefer-string-slice": "error",
			"unicorn/prefer-string-trim-start-end": "error", // Updated from prefer-trim-start-end
			"unicorn/prefer-dom-node-dataset": "error", // Updated from prefer-dataset
			"unicorn/custom-error-definition": "error",
			"unicorn/no-hex-escape": "error",
			"unicorn/no-process-exit": "error",
			"unicorn/no-new-buffer": "error",

			"@typescript-eslint/explicit-member-accessibility": [
				"error",
				{ accessibility: "no-public", overrides: { properties: "off" } },
			],
			"@typescript-eslint/explicit-function-return-type": [
				"error",
				{ allowConciseArrowFunctionExpressionsStartingWithVoid: true },
			],
			"@typescript-eslint/consistent-type-definitions": ["warn", "interface"],
			"@typescript-eslint/array-type": ["warn", { default: "generic" }],
			"@typescript-eslint/dot-notation": "off",
			"@typescript-eslint/consistent-type-imports": "warn",
			"new-cap": "off",

			"unicorn/catch-error-name": ["warn", { name: "error" }],
			"unicorn/prefer-spread": "off",
			"unicorn/no-negated-condition": "warn",
			"unicorn/no-array-callback-reference": "off",
			// "unicorn/no-null": "off", // I agree on principle, but some
			"unicorn/text-encoding-identifier-case": "off", // false positives in Astro HTML blocks
			"unicorn/no-nested-ternary": "off", // conflicts with Prettier
			"unicorn/prevent-abbreviations": [
				"warn",
				{
					allowList: {
						attrs: true,
						env: true,
						ref: true,
						rel: true,
						src: true,
						Props: true,
					},
				},
			],
		},
	},
	{
		files: ["**/*.astro"],

		languageOptions: {
			parser: parser,
			ecmaVersion: 5,
			sourceType: "script",

			parserOptions: {
				parser: typescriptEslint.parser,
				projectService: true, // FIXME: Astro's parser doesn't know how to handle this, so we miss some lints; see https://github.com/ota-meshi/astro-eslint-parser/issues/331
				tsconfigRootDir: import.meta.dirname,
				extraFileExtensions: [".astro"],
			},
		},

		rules: {},
	},
];
