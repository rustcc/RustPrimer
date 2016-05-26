# Spacemacs
spacemacs, it is a given vimer of emacs.
## Introduction
spacemacs is dedicated to those accustomed to the operation of vim, emacs while longing for expansion ability. It is very frustrating for me this had vim, emacs configuration over the people, but it also welcomes any newcomer no basis to use. In simple terms, it is a box with Emacs! This one is easier than a lot of people and not all of them are big things for the software.

## Installation
As the author himself in linux platform, and no experience windows platforms, so here it is not incompetence, I look forward to add. In addition, windows platform really need it, look askance to the Visual Studio.

### Emacs installation

In * nix systems, it will not necessarily installed by default Emacs, even if installed, not necessarily the latest version. Here, I strongly recommend that you uninstall the system comes with Emacs, because you do not know the system you install is what strange existence, was the most heart, I had met only castrated version of Emacs linux distributions.

I suggest that their own home page to download the project to emacs emacs-24.5 (the latest version as of this writing) and above, and then download the source code. As emacs installation is very simple, linux platform old three steps.
`` `Bash
./configure
make
sudo make install
`` `
what? You did not make? No GCC? Missing Dependency?
Please install them ......

### Spacemacs installation

Said earlier, Spacemacs Emacs is a profile library, so we can install it through a very simple way:
`` `Bash
git clone https://github.com/syl20bnr/spacemacs ~ / .emacs.d
cd ~ / .emacs.d
echo $ (git describe --tags $ (git rev-list --tags --max-count = 1)) | xargs git checkout
touch ~ / .spacemacs
`` `
Among them, the three lines is the author added, this must be what is Tucao, Spacemacs the master branch and is actually behind and wrong! Its current release are playing from the develop branch tag.

Thus, a! set! Do not! want! use! the Lord! Minute! support!

Finally, the reason you want to add the last line, which is when I installed the release of a small bug, do not have this document, then, emacs does not successfully complete initialization.

Well, we have to get the configuration file, then start your emacs, spacemacs will automatically go online to download plug-ins you need to install the package. In addition, to bring their own ladders best, because what you want is not under, but the network is indeed anxious to catch.

### Preparation

In order to support Spacemacs Rust, we need a little configuration. First, please refer to [preparation] (../ 03-editors / 03-01-before.md), installed on your racer.

Here, it is strongly recommended that the racer environment variable is added to the system variables (they typically configured in `/ etc / profile /` Lane) and restart the system, because it is really a lot of people click on the icon to start emacs it, this is likely to lead emacs does not inherit their environment variables, which is very frustrating.

## To complete the configuration

### Spacemacs modify the standard configuration.

Spacemacs document provides a standard spacemacs [Profile] (https://github.com/syl20bnr/spacemacs/blob/master/core/templates/.spacemacs.template), you can add it to your own `~ / .spacemacs` file.

Here, we need to modify the custom plug on its part:
`` `Lisp
(Defun dotspacemacs / layers ()
  "Configuration Layers declaration.
You should not put any user code in this function besides modifying the variable
values. "
  (Setq-default
   ;; Base distribution to use. This is a layer contained in the directory
   ;; `+ Distribution '. For now available distributions are` spacemacs-base'
   ;; Or `spacemacs '. (Default' spacemacs)
   dotspacemacs-distribution 'spacemacs
   ;; List of additional paths where to look for configuration layers.
   ;; Paths must have a trailing slash (i.e. `~ / .mycontribs / ')
   dotspacemacs-configuration-layer-path '()
   ;; List of configuration layers to load. If it is the symbol `all 'instead
   ;; Of a list then all discovered layers will be installed.
   dotspacemacs-configuration-layers
   '(
     ------------------------------------------------ ;; ----------------
     ;; Example of useful layers you may want to use right away.
     ;; Uncomment some layer names and press <SPC f e R> (Vim style) or
     ;; <M-m f e R> (Emacs style) to install them.
     ------------------------------------------------ ;; ----------------
     auto-completion
     better-defaults
     git
     spell-checking
     syntax-checking
     version-control
     rust
     )
   ;; List of additional packages that will be installed without being
   ;; Wrapped in a layer. If you need some configuration for these
   ;; Packages then consider to create a layer, you can also put the
   ;; Configuration in `dotspacemacs / config '.
   dotspacemacs-additional-packages' ()
   ;; A list of packages and / or extensions that will not be install and loaded.
   dotspacemacs-excluded-packages' ()
   ;; If non-nil spacemacs will delete any orphan packages, i.e. packages that
   ;; Are declared in a layer which is not a member of
   ;; The list `dotspacemacs-configuration-layers'. (Default t)
   dotspacemacs-delete-orphan-packages t))

;; ...
The following configuration file contents omitted ;;
;; ...
`` `

Note that `dotspacemacs-configuration-layers` different configurations and standard configuration files.

Save the configuration file, and then restart your emacs, of course, we can also press the `SPC f e R` to accomplish the purpose reload the configuration file, then you will find emacs will start the next round of the download, wait its completion.

In the previous step, we have completed the configuration of the Racer environment variables, so now you have configured Spacemacs complete! This simple configurations, and almost compete with the Atom.

### Key Bindings
Below, spacemacs provides several default key bindings, but I do not think these useful, or prefer to use the command line.

| Key Binding | Description |
| ------------- | ----------------------------------- |
| ~ SPC m c c ~ | compile project with Cargo |
| ~ SPC m c t ~ | run tests with Cargo |
| ~ SPC m c d ~ | generate documentation with Cargo |
| ~ SPC m c x ~ | execute the project with Cargo |

Try ##

For now, we can open a Cargo project, and to use it. You will be surprised to find racer / flycheck / company three plug when mated together in harmony is so simple.
