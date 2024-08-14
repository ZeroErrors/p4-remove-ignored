# Perforce Remove Ignored
A simple tool to help remove files from a perforce depot that should be ignored by P4IGNORE.

## Usage
This tool can either be used as a standalone CLI tool or as a configured P4V Custom Tool.

### Configure P4V Custom Tool
1. Clone the repo
1. Run `cargo install --path .`
1. Open P4V
1. Go to `Tools -> Manage Tools -> Custom Tools...`
1. Click `Import Custom Tools...` in the bottom left corner
1. Navigate to and open [docs/pr-remove-ignored.xml](docs/pr-remove-ignored.xml)
1. Click `Import` then `OK` to close the Manage Custom Tools window 
1. Either in the Depot or Workspace view, select and right-click on any file(s) or folder(s) you wish the tool to check then select `Remove Ignored` from the context menu
1. Enter any options and click `OK`
    - *Note: Its a good idea to use `-d` to do a dry-run and see what files will be removed before actually removing any files.*


### CLI Tool
1. Clone the repo
1. Run `cargo install --path .`
1. Run `p4-remove-ignored` as needed in your depot
```
> p4-remove-ignored.exe --help
Remove files from a perforce depot that should be ignored by P4IGNORE.

Usage: p4-remove-ignored.exe [OPTIONS] <DEPOT_PATHS>...

Arguments:
  <DEPOT_PATHS>...  The depot paths to remove ignored files from

Options:
  -p, --port <PORT>      The Perforce port to connect to
  -u, --user <USER>      The Perforce user to connect as
  -c, --client <CLIENT>  The Perforce client workspace to use
  -d, --dry-run
  -k, --keep-files
  -h, --help             Print help
  -V, --version          Print version
```
