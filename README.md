# crisp

crisp is a fast, rule-based, probably naive JavaScript minifier. It operates without fully parsing JavaScript, using a lexer and rule-based decision making

## limitations

The main limitation of `crisp` is that it cannot mangle your code (ex. change long variable names). This is due to the fact that it does not parse the javascript, and thus cannot change variable names in any complex manner.

## usage

You *should* be able to use crisp using `npx`, like this:

```sh
npx crispify <files>
```