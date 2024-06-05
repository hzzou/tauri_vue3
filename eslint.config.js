import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginVue from "eslint-plugin-vue";


export default [
	pluginJs.configs.recommended,
	...tseslint.configs.recommended,
	...pluginVue.configs["flat/essential"],
	{
		languageOptions: {
			globals: globals.browser,
		},
		rules: {
			"@typescript-eslint/no-unused-vars": "off",
			"vue/multi-word-component-names": "off",
			"no-unused-vars": "off",
			"indent": [
				"warn",
				"tab"
			],
			"linebreak-style": [
				"error",
				"unix"
			],
			"quotes": [
				"error",
				"double"
			],
			"semi": [
				"error",
				"always"
			],
		}
	},
];
