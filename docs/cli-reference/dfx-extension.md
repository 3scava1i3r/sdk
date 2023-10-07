# dfx extension

Use the `dfx extension` command to manage the extensions available in the `dfx` tool. Extensions can provide additional functionality and commands to the `dfx` tool, enhancing its capabilities.

## Basic Usage

```bash
dfx extension [command] [option] [flag]
```

## Commands

| Command    | Description                                           |
|------------|-------------------------------------------------------|
| [`install`](#dfx-extension-install)    | Installs an extension.                                |
| [`uninstall`](#dfx-extension-uninstall)  | Uninstalls an extension.                              |
| [`run`](#dfx-extension-run)        | Executes an extension.                                |
| [`list`](#dfx-extension-list)       | Lists all installed extensions.                       |

## dfx extension install
## dfx extension uninstall
## dfx extension run
## dfx extension list

1. **Installing an Extension**:

   ```bash
   dfx extension install sns
   ```

   This command will install the `sns` extension.

2. **Listing Installed Extensions**:

   ```bash
   dfx extension list
   ```

   This command will display all the installed extensions.

3. **Uninstalling an Extension**:

   ```bash
   dfx extension uninstall sns
   ```

   This command will uninstall the `sns` extension.

4. **Running an Extension**:

   If an extension provides executable commands, you can run them using:

   ```bash
   dfx extension run <EXTENSION_NAME> <COMMAND>
   ```

5. **Displaying Help for an Extension**:

   ```bash
   dfx extension help
   ```

   This command will display detailed help for the `dfx extension` command, including its subcommands and their options.

