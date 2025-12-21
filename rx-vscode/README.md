# Reactive Language Support for VS Code

## VS Code language support for the Reactive programming language.

### Installation (Manual)

This extension is currently installed manually.

- Open "Extensions" tab
- Click "..." and press "Install from VSIX"
- Navigate to Reactive/rx-vscode folder and select "rx-vscode-0.0.1.vsix"

Syntax highlighting should activate automatically

Alternatively, you can package and install it:
```
vsce package
```

Then install the generated .vsix file via:
```
Extensions -> Install from VSIX…
```

### File Association

If needed, add this to your VS Code settings to associate files:
```
"files.associations": {
  "*.rx": "reactive"
}
```

