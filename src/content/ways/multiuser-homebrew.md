---
title: "How to convert Homebrew for Mac to work with multiple users"
description: "If you've already installed Homebrew on a single-user machine, and want to use Homebrew on a second user, the migration may be complicated."
date: "2024-03-18"
---

If you've already installed Homebrew on a single-user machine, and want to use Homebrew on a second user, the migration may be complicated.

For example, let's say you have `olduser` on your personal machine set up with Homebrew. You create a second user `newuser` and try to use `brew` and immediately run into permission issues. [Homebrew is not designed for multi-user setups](https://docs.brew.sh/FAQ#why-does-homebrew-say-sudo-is-bad) by default.

Instead, create a third user, `homebrew` for example, where Homebrew can keep its cache files and other local state, and alias the `brew` command to switch to that user for commands.

## Step 0: Write down your installed packages

First, write down the lists of casks and formulae you have installed through Homebrew:

```sh
brew leaves --installed-on-request
brew list --cask
```

> [!WARNING]
> This process will uninstall all Homebrew packages. You will need to re-install them manually after you've moved your Homebrew installation.

## Step 1: Create the Homebrew Manager user

Next, use System Settings to create a new Administrator user to use to manage Homebrew (`homebrew` in this example, with the long name "Homebrew Manager"). Then follow [Apple's instructions](https://support.apple.com/en-us/102099) to hide that user from the login window...

```sh
sudo dscl . create /Users/homebrew IsHidden 1
sudo defaults write /Library/Preferences/com.apple.loginwindow HiddenUsersList -array-add homebrew
```

...and hide its home directory:

```sh
sudo chflags hidden /Users/homebrew
```

After a reboot, the user should be hidden from the login screen.

> [!NOTE]
> If FileVault is enabled, the `homebrew` user may be visible on the boot screen.

## Step 2: Uninstall Homebrew

Follow [Homebrew's official uninstall instructions](https://github.com/Homebrew/install#uninstall-homebrew):

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/uninstall.sh)"
```

## Step 3: Reinstall Homebrew as the new user

Log in to the new user from your terminal:

```sh
login
# Enter your new user's username (e.g. `homebrew`) and password
```

Then [install Homebrew](https://github.com/Homebrew/install):

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Set up Homebrew on the new user's `PATH` variable:

```sh
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> /Users/homebrew/.zprofile
eval "$(/opt/homebrew/bin/brew shellenv)"
```

Then, override `HOMEBREW_CASK_OPTS` in the `homebrew` user to change Homebrew's Fonts directory from [the default value of `~/Library/Fonts`](https://github.com/Homebrew/homebrew-cask/blob/e1f76fe7a394dac52bfd60a8ed289560ae9c4992/USAGE.md#options) to the system fonts directory at `/Library/Fonts`:

```sh
echo 'export HOMEBREW_CASK_OPTS="--fontdir=/Library/Fonts"' >> /Users/homebrew/.zprofile
```

Finally, log out:

```sh
logout
```

## Step 4: Alias the `brew` command to run as the `homebrew` user

Add the following to wherever you keep your shell's terminal aliases:

```sh
alias brew='sudo -Hu homebrew brew'
```

## Step 5: Reinstall everything

Use `brew install` to reinstall everything in the list you wrote down before. Since `brew` aliases to `sudo` now, you may need to enter your password [or use TouchID](https://apple.stackexchange.com/a/466029) to run `brew` commands.

---

You are now free to create new users as you please on your system, and Homebrew will behave appropriately!

These instructions are adapted from [Valérian Galliat's blog post](https://www.codejam.info/2021/11/homebrew-multi-user.html).
